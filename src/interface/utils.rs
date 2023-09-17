use ratatui::{
    prelude::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use ratatui_textarea::TextArea;

/// Returns the height of the input.
pub fn get_input_height(lines: usize, min_height: usize, max_height: usize) -> u16 {
    std::cmp::min(std::cmp::max(lines, min_height), max_height) as u16 + 2
}

/// Create the help message of the interface. from a list of keys and descriptions.
pub fn create_interface_help_message(keys: &[(&str, &str)], width: u16) -> String {
    let mut length = 0;
    let mut is_first = true;
    let mut help_message = String::from("Press ");
    keys.iter().for_each(|(key, description)| {
        length += key.len() as u16 + description.len() as u16 + 3;
        if length >= width - 10 {
            length = 0;
            help_message.push_str(&format!("\n`{key}` to {description}"))
        } else {
            let spliter = if is_first {
                is_first = false;
                String::new()
            } else {
                ", ".to_string()
            };
            help_message.push_str(&format!("{spliter}`{key}` to {description}"))
        }
    });
    help_message + crate::app::main_help()
}

/// Create the help block of the interface. from the help message.
pub fn create_interface_help_block(help_message: String) -> impl Widget {
    Paragraph::new(help_message)
        .block(
            Block::default()
                .style(Style::default().bg(Color::Cyan).fg(Color::Black))
                .title("Help")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().bg(Color::Cyan).fg(Color::Black)),
        )
        .alignment(Alignment::Left)
}

/// Create the textarea block. and set the title.
pub fn crate_textarea_block(title: String) -> TextArea<'static> {
    let mut text_area = TextArea::default();
    text_area.set_cursor_line_style(Style::default());
    text_area.set_block(
        Block::default()
            .title(title)
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default()),
    );

    text_area
}

/// Create the textarea widget. and set the block style according to the write mode.
pub fn textarea_widget<'a>(
    text_area: &'a mut TextArea<'static>,
    is_write_mode: bool,
) -> impl Widget + 'a {
    let block = text_area
        .block()
        .expect("the text area has a block")
        .clone();
    if is_write_mode {
        text_area.set_cursor_style(Style::default().bg(Color::White));
        text_area.set_block(block.border_style(Style::default().fg(Color::Cyan)));
    } else {
        text_area.set_cursor_style(Style::default());
        text_area.set_block(
            block
                .clone()
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    }
    text_area.widget()
}

/// Create a paragraph widget.
pub fn paragraph_widget<'a>(title: &'a str, text: &'a str) -> impl Widget + 'a {
    Paragraph::new(text)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(Alignment::Left)
}
