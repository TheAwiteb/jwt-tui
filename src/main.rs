use directories::ProjectDirs;
use jwt_tui::app::{App, AppResult};
use jwt_tui::event::{Event, EventHandler};
use jwt_tui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use simplelog::*;
use std::io;

fn init_logger() {
    let log_dir = ProjectDirs::from("com", "jwt-tui", "jwt-tui")
        .expect("failed to get project directories")
        .data_dir()
        .to_owned()
        .join("logs");

    std::fs::create_dir_all(&log_dir).expect("failed to create log directory");
    let log_file = log_dir.join(format!(
        "jwt-tui-{}.log",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("The system clock is set to before the UNIX epoch!")
            .as_secs()
    ));

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        std::fs::File::create(log_file).expect("failed to create log file"),
    )])
    .expect("failed to initialize logger");
}

fn main() -> AppResult<()> {
    // Initialize the logger.
    init_logger();
    log::info!("Starting jwt-tui");

    if std::env::args().any(|arg| arg == "--version" || arg == "-v") {
        println!("jwt-tui v{}", env!("CARGO_PKG_VERSION"));
    } else {
        // Create an application.
        let mut app = App::new();

        // Initialize the terminal user interface.
        let backend = CrosstermBackend::new(io::stderr());
        let terminal = Terminal::new(backend)?;
        let events = EventHandler::new(250);
        let mut tui = Tui::new(terminal, events);
        tui.init()?;
        log::info!("Initialized the terminal user interface");

        // Start the main loop.
        while app.running {
            // Render the user interface.
            tui.draw(&mut app)?;
            // Handle events.
            match tui.events.next()? {
                Event::Key(key_event) => app.handle_key_event(key_event)?,
                Event::Tick => {
                    // This gets called every 250ms if no key is pressed.
                    // This will rerender the UI if the app is running.
                }
            }
        }

        // Exit the user interface.
        tui.exit()?;
    }
    Ok(())
}
