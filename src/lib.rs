/// The minimum terminal supported height.
pub const MIN_TERMINAL_HEIGHT: u16 = 26;
/// The minimum terminal supported width.
pub const MIN_TERMINAL_WIDTH: u16 = 76;

/// Application.
pub mod app;

/// Interfaces of the application.
pub mod interface;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Widgets.
pub mod widgets;
