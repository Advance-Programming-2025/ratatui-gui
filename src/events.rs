use crossterm::event::{self, Event, KeyCode};

use crate::app::App;
use crate::states::GameState;

pub fn handle_events(app: &mut App) -> Result<(), String> {
    if event::poll(app.tick_rate).map_err(|_| "Polling error")? {
        if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
            match (key.code, app.get_game_state()) {
                (KeyCode::Char('q'), _) => {
                    app.exit = true;
                }
                (KeyCode::Char('s'), GameState::WaitingStart) => {
                    app.set_game_state(GameState::Running);
                    app.orchestrator.start_all()?;
                }
                (KeyCode::Char('p'), GameState::Running) => {
                    app.set_game_state(GameState::Paused);
                }
                (KeyCode::Char('p'), GameState::Paused) => {
                    app.set_game_state(GameState::Running);
                }
                _ => {}
            }
        }
    }
    Ok(())
}