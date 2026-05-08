/// Game UI screens and render entrypoints.
mod main_screen;
/// Start screen and pre-game UI.
mod screens;
/// Centralized layout constraints for tables.
mod layout;
/// Theme and styling utilities.
pub mod theme;

use ratatui::Frame;

use crate::app::App;
use crate::game_state::GameState;

/// UI render entrypoint.
/// Selects active screen based on `GameState` and passes theme context down.
pub fn render_ui(app: &mut App, frame: &mut Frame) {
    let theme = theme::Theme::matrix();
    match app.get_game_state() {
        GameState::WaitingStart => {
            // Show start screen
            screens::render_start_screen(app, frame, &theme);
        }
        GameState::Running => {
            // Show normal game UI
            main_screen::render_game_ui(app, frame, &theme);
        }
        GameState::Paused => {
            // Show game UI with pause overlay
            main_screen::render_game_ui(app, frame, &theme);
        }
        GameState::Ended => {
            // Show game UI in ended state
            main_screen::render_game_ui(app, frame, &theme);
        }
    }
}
