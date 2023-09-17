use ratatui::{prelude::*, Frame};

use super::{utils, Interface};

/// The encode interface. This interface is used to encode a JWT.
#[derive(Debug, impl_new::New)]
pub struct EncodeInterface {}

impl Interface for EncodeInterface {
    fn name(&self) -> &'static str {
        "Encode"
    }

    fn help(&self, width: u16) -> String {
        utils::create_interface_help_message(
            &[
                ("I", "Write data"),
                ("S", "Write secret key"),
                ("ESC", "Quit write mode"),
            ],
            width,
        )
    }

    fn render(
        &mut self,
        frame: &mut Frame<'_, CrosstermBackend<std::io::Stderr>>,
        interface_idx: usize,
    ) {
        let size = frame.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(super::chunks_constraints(
                self,
                size.width,
                [Constraint::Min(7)].as_ref(),
            ))
            .split(size);

        // ...
        super::main_widgets(self, frame, interface_idx, chunks);
    }

    fn key_handler(&mut self, _key: crossterm::event::KeyEvent) {}
}
