use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration};

/// Represents the different states the game can be in
#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    /// Waiting for the player to start the game
    WaitingStart,
    /// Game is actively running
    Running,
    /// Game is paused
    Paused,
    /// Game has ended
    Ended,
}

/// Handles user input and updates the game state accordingly
///
/// Polls for keyboard events and processes them based on the current game state.
/// Only processes Press events to avoid key repeat issues.
pub fn handle_game_state(app: &mut App) -> Result<(), String> {
        // Very short timeout for responsive input
        if event::poll(Duration::from_millis(10)).map_err(|_| "Polling error")? {
            if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
                match (key.code, app.get_game_state()) {
                    // Global events - respond immediately on key press
                    (KeyCode::Char('q'), _) => {
                        app.exit = true;
                    }
                    (KeyCode::Enter, GameState::WaitingStart) => {
                        app.set_game_state(GameState::Running);
                        app.orchestrator.start_all()?;
                    }
                    (KeyCode::Char('p'), GameState::Running) => {
                        app.set_game_state(GameState::Paused);
                    }
                    (KeyCode::Char('p'), GameState::Paused) => {
                        app.set_game_state(GameState::Running);
                    }
                    (KeyCode::Up, _) => app.set_sunray_increment(),
                    (KeyCode::Down, _) => app.set_sunray_decrement(),

                    // Navigation events
                    (KeyCode::Char('w'), _) => {
                        app.decrement_id_selector();
                    }
                    (KeyCode::Char('s'), _) => {
                        app.increment_id_selector();
                    }

                    // Toggle log overlay with 'L'
                    (KeyCode::Char('l'), _) => {
                        app.show_log_overlay = !app.show_log_overlay;
                    }

                    // Restart game when ended
                    (KeyCode::Char('r'), GameState::Ended) => {
                        app.set_game_state(GameState::WaitingStart);
                        // TODO: Add orchestrator reset function
                    }
                    _ => {}
                }
            }
    }
    Ok(())
}
