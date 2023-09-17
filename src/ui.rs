use ratatui::{prelude::CrosstermBackend, Frame};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame<'_, CrosstermBackend<std::io::Stderr>>) {
    let size = frame.size();
    if size.width < crate::MIN_TERMINAL_WIDTH || size.height < crate::MIN_TERMINAL_HEIGHT {
        frame.render_widget(crate::widgets::SmallWindowSize(size), size);
    } else {
        let interface_idx = app.current_interface_index;
        let current_interface = app.current_interface_mut();
        current_interface.render(frame, interface_idx);
    }
}
