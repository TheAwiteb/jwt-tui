use crossterm::event::KeyEvent;
use ratatui::{prelude::*, Frame};
use ratatui_textarea::{Input, Key, TextArea};

use super::{utils, Interface};

/// The decode interface. This interface is used to decode and verify a JWT.
#[derive(impl_new::New)]
pub struct DecodeInterface {
    /// The JWT.
    #[impl_new(value = || utils::crate_textarea_block("JWT".to_owned()))]
    pub jwt: TextArea<'static>,
    /// The secret key.
    #[impl_new(value = || utils::crate_textarea_block("Secret key".to_owned()))]
    pub secret_key: TextArea<'static>,
    /// The decoded JWT.
    #[impl_new(value = || String::new())]
    pub decoded_jwt: String,
    /// Is jwt in write mode?
    #[impl_new(value = || false)]
    pub jwt_write_mode: bool,
    /// Is secret key in write mode?
    #[impl_new(value = || false)]
    pub secret_key_write_mode: bool,
}

impl DecodeInterface {
    /// Quit the write mode.
    pub fn quit_write_mode(&mut self) {
        self.jwt_write_mode = false;
        self.secret_key_write_mode = false;
    }
}

impl Interface for DecodeInterface {
    fn name(&self) -> &'static str {
        "Decode"
    }

    fn help(&self, width: u16) -> String {
        utils::create_interface_help_message(
            &[
                ("I", "Write JWT"),
                ("S", "Write secret key"),
                ("ESC", "Quit write mode"),
            ],
            width,
        )
    }

    fn key_handler(&mut self, key: KeyEvent) {
        match key.into() {
            Input {
                key: Key::Char('I'),
                ..
            }
            | Input {
                key: Key::Char('i'),
                ..
            } if !self.in_write_mode() => {
                log::info!("Decode interface: Entering in JWT write mode");
                self.jwt_write_mode = true;
                self.secret_key_write_mode = false;
            }
            Input {
                key: Key::Char('S'),
                ..
            }
            | Input {
                key: Key::Char('s'),
                ..
            } if !self.in_write_mode() => {
                log::info!("Decode interface: Entering in secret key write mode");
                self.jwt_write_mode = false;
                self.secret_key_write_mode = true;
            }
            Input { key: Key::Esc, .. } => {
                log::info!("Decode interface: Quitting write mode");
                self.quit_write_mode()
            }
            Input { key, alt, ctrl } => {
                let input = Input { key, alt, ctrl };
                if self.jwt_write_mode {
                    self.jwt.input(input);
                } else if self.secret_key_write_mode {
                    self.secret_key.input(input);
                }
            } // KeyCode::Backspace => {
              //     if self.jwt_write_mode {
              //         self.jwt.pop();
              //     } else if self.secret_key_write_mode {
              //         self.secret_key.pop();
              //     }
              // }
        }
    }

    fn render(
        &mut self,
        frame: &mut Frame<'_, CrosstermBackend<std::io::Stderr>>,
        interface_idx: usize,
    ) {
        let size = frame.size();

        let jwt_height = utils::get_input_height(
            self.jwt.lines().len(),
            super::MIN_INPUT_HEIGHT,
            super::MAX_INPUT_HEIGHT,
        );

        let decoded_jwt_height = utils::get_input_height(
            self.decoded_jwt.lines().count(),
            super::MIN_TEXT_HEIGHT,
            super::MAX_TEXT_HEIGHT,
        );

        let secret_key_height = utils::get_input_height(
            self.secret_key.lines().len(),
            super::MIN_INPUT_HEIGHT,
            super::MAX_INPUT_HEIGHT,
        );

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(super::chunks_constraints(
                self,
                size.width,
                [
                    Constraint::Max(jwt_height),
                    Constraint::Max(secret_key_height),
                    Constraint::Min(decoded_jwt_height),
                ]
                .as_ref(),
            ))
            .split(size);

        frame.render_widget(
            utils::textarea_widget(&mut self.jwt, self.jwt_write_mode),
            chunks[1],
        );
        frame.render_widget(
            utils::textarea_widget(&mut self.secret_key, self.secret_key_write_mode),
            chunks[2],
        );
        frame.render_widget(
            utils::paragraph_widget("Decoded JWT", &self.decoded_jwt),
            chunks[3],
        );
        super::main_widgets(self, frame, interface_idx, chunks);
    }

    fn in_write_mode(&self) -> bool {
        self.jwt_write_mode || self.secret_key_write_mode
    }
}
