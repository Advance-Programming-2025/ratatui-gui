use std::time::Duration;

use crate::app::App;
use crossterm::event::{self, Event, KeyCode};

#[derive(Clone, PartialEq)]
pub enum GameState {
    WaitingStart,
    Running,
    Paused,
}

pub fn handle_game_state(app: &mut App) -> Result<(), String> {
    if event::poll(Duration::from_millis(10)).map_err(|_| "Polling error")? {
        if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
            match (key.code, app.get_game_state()) {
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
                (KeyCode::Char('w'), GameState::Running) => {
                    app.decrement_id_selector();
                }
                (KeyCode::Char('s'), GameState::Running) => {
                    app.increment_id_selector();
                }
                _ => {}
            }
        }
    }
    Ok(())
}
