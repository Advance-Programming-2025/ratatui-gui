use crate::{app::App, commands::Command, controller::Controller, input, ui_state::UiMode};
use crossterm::event::{self, Event};
use std::time::Duration;

/// Represents the different states the game can be in
#[derive(Clone, PartialEq, Debug)]
pub(crate) enum GameState {
    /// Waiting for the player to start the game
    WaitingStart,
    /// Game is actively running
    Running,
    /// Game is paused
    Paused,
    /// Game has ended
    Ended,
}

/// Global input handler that bridges TUI events with game logic.
/// Reads one key event, dispatches to controller, then applies emitted commands.
pub(crate) fn handle_game_state(app: &mut App) -> Result<(), String> {
    // Very short timeout for responsive input
    if event::poll(Duration::from_millis(5)).map_err(|_| "Polling error")? {
        if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
            let action = input::map_key(key);
            let mut controller = Controller;
            let transition = controller.handle(app, action);
            for command in transition.commands {
                apply_command(app, command)?;
            }
        }
    }
    Ok(())
}

fn apply_command(app: &mut App, command: Command) -> Result<(), String> {
    match command {
        Command::SetGameState(state) => {
            app.set_game_state(state);
        }
        Command::StartGame => {
            let mattia_explorers = vec![(0, 0)];
            let tommy_explorers = vec![(1, 1)];
            app.orchestrator
                .start_all(&mattia_explorers, &tommy_explorers)?;
            app.set_game_state(GameState::Running);
            app.get_game_info()?;
            app.reset_sim_clock(std::time::Instant::now());
        }
        Command::StopAll => {
            app.orchestrator.stop_all()?;
        }
        Command::RestartAll => {
            app.orchestrator.restart_all()?;
        }
        Command::StopExplorerAI { explorer_id } => {
            app.orchestrator.send_stop_explorer_from_gui(explorer_id)?;
        }
        Command::ToggleLog => {
            app.ui.overlays.show_log = !app.ui.overlays.show_log;
        }
        Command::AdjustCustomPlanets(delta) => {
            if app.ui.start.selected_mode == 1 {
                app.adjust_custom_planets(delta);
            }
        }
        Command::ToggleGenerationMode => {
            app.toggle_generation_mode();
        }
        Command::QueueAsteroid { planet_id } => {
            app.add_incoming_asteroid_for_planet(planet_id);
        }
        Command::QueueSunray { planet_id } => {
            app.add_incoming_sunray_for_planet(planet_id);
        }
        Command::MoveExplorer {
            explorer_id,
            planet_id,
        } => {
            app.orchestrator
                .send_move_explorer_from_gui(explorer_id, planet_id)?;
            app.ui.mode = UiMode::Normal;
        }
    }

    Ok(())
}
