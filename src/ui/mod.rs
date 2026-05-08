mod main_screen;
mod screens;
mod layout;
pub mod theme;

use ratatui::Frame;

use crate::app::App;
use crate::game_state::GameState;

/// Primary entry point for rendering.
/// Dispatches the frame to specific screens (Start, Running, Paused) based on GameState.
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
