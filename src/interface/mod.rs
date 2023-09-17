/// Decode interface.
pub mod decode;
/// Encode interface.
pub mod encode;
/// Interfaces utils.
pub mod utils;

use crossterm::event::KeyEvent;
pub use decode::*;
pub use encode::*;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

pub const MIN_INPUT_HEIGHT: usize = 2;
pub const MAX_INPUT_HEIGHT: usize = 4;

pub const MIN_TEXT_HEIGHT: usize = 10;
pub const MAX_TEXT_HEIGHT: usize = 10;

/// The interface trait. This trait is used to implement the interfaces of the application.
/// Help us to manage the interfaces of the application.
pub trait Interface: Send + Sync {
    /// The name of the interface.
    fn name(&self) -> &'static str;

    /// The help message of the interface.
    fn help(&self, width: u16) -> String;

    /// The key handler of the interface.
    fn key_handler(&mut self, key: KeyEvent);

    /// Render the interface.
    fn render(
        &mut self,
        frame: &mut Frame<'_, CrosstermBackend<std::io::Stderr>>,
        interface_idx: usize,
    );

    /// Returns whether the interface is in write mode or not.
    fn in_write_mode(&self) -> bool {
        false
    }
}

/// Returns the interfaces of the application as a [`Interface`] trait object vector.
pub fn get_interfaces() -> Vec<Box<dyn Interface>> {
    vec![
        Box::new(DecodeInterface::new()),
        Box::new(EncodeInterface::new()),
    ]
}

/// Returns the interfaces names.
pub const fn get_interfaces_names() -> [&'static str; 2] {
    ["Decode", "Encode"]
}

/// Returns the interfaces menu widget.
pub fn create_interfaces_menu_widget(current_interface_index: usize) -> impl Widget + 'static {
    let mut items = Vec::new();
    for (index, interface) in get_interfaces_names().into_iter().enumerate() {
        if index != 0 {
            items.push(Span::styled(
                " | ",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        items.push(Span::styled(
            interface.to_string(),
            Style::default()
                .fg(if index == current_interface_index {
                    Color::Cyan
                } else {
                    Color::Gray
                })
                .add_modifier(Modifier::BOLD),
        ));
    }

    let menu = Paragraph::new(Line::from(items)).block(
        Block::default()
            .borders(Borders::ALL)
            .fg(Color::DarkGray)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Color::DarkGray))
            .bold(),
    );
    menu
}

/// Add the main widget of any interface to the frame.
/// This will add
/// - The interfaces menu. (top)
/// - The help block. (bottom)
pub fn main_widgets(
    interface: &mut dyn Interface,
    frame: &mut Frame<'_, CrosstermBackend<std::io::Stderr>>,
    interface_idx: usize,
    chunks: std::rc::Rc<[Rect]>,
) {
    let help_chunk = *chunks.last().expect("There is at least one chunk!");
    let help_message = interface.help(help_chunk.width);
    frame.render_widget(create_interfaces_menu_widget(interface_idx), chunks[0]);
    frame.render_widget(utils::create_interface_help_block(help_message), help_chunk);
}

/// Constraints for the chunks of the interface. This will return the constraints of the chunks of the interface.
/// - The first chunk is the menu chunk.
/// - The last chunk is the help chunk.
/// - The middle chunks is the given chunks.
pub fn chunks_constraints(
    interface: &mut dyn Interface,
    width: u16,
    middle_chunks: &[Constraint],
) -> Vec<Constraint> {
    let help_message = interface.help(width);
    let help_height = utils::get_input_height(help_message.lines().count(), 1, 3);

    let mut chunks = Vec::new();
    chunks.push(Constraint::Length(3));
    chunks.extend(middle_chunks);
    chunks.push(Constraint::Length(help_height));
    chunks
}
