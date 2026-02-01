mod main_screen;
mod screens;

use ratatui::Frame;

use crate::app::App;
use crate::game_state::GameState;

/// Main UI rendering dispatcher
///
/// Routes to the appropriate screen based on current game state:
/// - WaitingStart: Shows start screen
/// - Running: Shows main game UI
/// - Paused: Shows main game UI (pause overlay can be added if needed)
/// - Ended: Shows main game UI with end state
pub fn render_ui(app: &mut App, frame: &mut Frame) {
    match app.get_game_state() {
        GameState::WaitingStart => {
            // Show start screen
            screens::render_start_screen(app, frame);
        }
        GameState::Running => {
            // Show normal game UI
            main_screen::render_game_ui(app, frame);
        }
        GameState::Paused => {
            // Show game UI with pause overlay
            main_screen::render_game_ui(app, frame);
        }
        GameState::Ended => {
            // Show game UI in ended state
            main_screen::render_game_ui(app, frame);
        }
    }
}
