use std::error;

use crate::interface::{get_interfaces, Interface};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(impl_new::New)]
pub struct App {
    /// Is the application running?
    #[impl_new(value = || true)]
    pub running: bool,
    /// The interfaces of the application.
    #[impl_new(value = || get_interfaces())]
    pub interfaces: Vec<Box<dyn Interface>>,
    /// The current interface index.
    #[impl_new(value = || 0)]
    pub current_interface_index: usize,
}

impl App {
    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Returns the current interface as a reference.
    pub fn current_interface(&self) -> &dyn Interface {
        &*self.interfaces[self.current_interface_index]
    }

    /// Returns the current interface as a mutable reference.
    pub fn current_interface_mut(&mut self) -> &mut dyn Interface {
        &mut *self.interfaces[self.current_interface_index]
    }

    /// Increment the current interface index.
    /// If the current interface index is the last index, set it to 0.
    pub fn next_interface(&mut self) {
        if self.current_interface_index == self.interfaces.len() - 1 {
            self.current_interface_index = 0;
        } else {
            self.current_interface_index += 1;
        }
    }

    /// Decrement the current interface index.
    /// If the current interface index is 0, set it to the last index.
    pub fn previous_interface(&mut self) {
        if self.current_interface_index == 0 {
            self.current_interface_index = self.interfaces.len() - 1;
        } else {
            self.current_interface_index -= 1;
        }
    }

    /// Handle the key event.
    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> AppResult<()> {
        match key_event.code {
            // Exit application on `ESC` or `q`
            crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Char('Q')
                if !self.current_interface().in_write_mode() =>
            {
                log::info!("Exit application");
                self.quit();
            }
            // Next interface on `TAB`
            crossterm::event::KeyCode::Tab => {
                log::info!("Next interface");
                self.next_interface();
            }
            // Previous interface on `SHIFT + TAB`
            crossterm::event::KeyCode::BackTab => {
                log::info!("Previous interface");
                self.previous_interface();
            }
            // Handle the key event of the current interface
            _ => {
                self.current_interface_mut().key_handler(key_event);
            }
        }
        Ok(())
    }
}

/// Returns the main help message.
pub const fn main_help() -> &'static str {
    "\n`q` to quit the application, `TAB` to switch interface"
}
