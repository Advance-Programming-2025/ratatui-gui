mod app;
mod game_state;
mod loops;
mod tui_loggers;
mod ui;

use std::sync::Arc;

use crate::tui_loggers::LogBuffer;
use crate::tui_loggers::SimpleTuiLogger;
use app::App;
use omc_galaxy::Orchestrator;

fn main() -> Result<(), String> {
    let log_buffer = Arc::new(LogBuffer::new(50)); // Ultimi 50 messaggi
    let logger = SimpleTuiLogger {
        buffer: Arc::clone(&log_buffer),
    };

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info); // Imposta il livello desiderato
    // Init terminal
    let mut terminal = ratatui::init();

    // Init orchestrator
    let orchestrator = Orchestrator::new()?;

    // Create and run game loop
    let mut app = App::new(orchestrator, log_buffer);

    // Initialize by file
    app.initialize_by_file()?;

    // Start the app
    let result = app.run(&mut terminal);

    // Restore terminal
    ratatui::restore();

    // Return possible error from the app run
    result
}
