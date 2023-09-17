use ratatui::{
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

#[derive(Clone, impl_new::New)]
pub struct SmallWindowSize(#[impl_new(name = "current_size")] pub ratatui::prelude::Rect);

impl Widget for SmallWindowSize {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::buffer::Buffer) {
        Paragraph::new(format!(
            "Very small window size: {:?}\nThe minimum window size is {}x{}.",
            self.0,
            crate::MIN_TERMINAL_WIDTH,
            crate::MIN_TERMINAL_HEIGHT
        ))
        .alignment(ratatui::prelude::Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .block(
            ratatui::widgets::Block::default()
                .title("Error")
                .title_alignment(ratatui::prelude::Alignment::Center)
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Plain)
                .border_style(Style::default().fg(Color::Red)),
        )
        .render(area, buf)
    }
}
