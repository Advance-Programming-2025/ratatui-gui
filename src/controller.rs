//! UI controller state machine.
//! Dispatches actions using nested (phase, mode, focus) state.

use crate::{
    app::App,
    commands::Command,
    game_state::GameState,
    input::Action,
    ui_state::{Focus, UiMode},
};

/// Result of one controller step.
pub struct Transition {
    pub commands: Vec<Command>,
}

impl Transition {
    fn none() -> Self {
        Self { commands: vec![] }
    }

    fn one(command: Command) -> Self {
        Self {
            commands: vec![command],
        }
    }
}

/// UI controller owning input-to-state transitions.
pub struct Controller;

impl Controller {
    /// Handle one semantic action.
    pub fn handle(&mut self, app: &mut App, action: Action) -> Transition {
        if matches!(action, Action::None) {
            return Transition::none();
        }

        if matches!(action, Action::Quit) {
            app.exit = true;
            return Transition::none();
        }

        match app.get_game_state() {
            GameState::WaitingStart => self.handle_start(app, action),
            GameState::Running | GameState::Paused => self.handle_in_game(app, action),
            GameState::Ended => self.handle_ended(app, action),
        }
    }

    fn handle_start(&mut self, _app: &mut App, action: Action) -> Transition {
        match action {
            Action::Confirm => Transition::one(Command::StartGame),
            Action::Up | Action::Down => Transition::one(Command::ToggleGenerationMode),
            Action::Left => Transition::one(Command::AdjustCustomPlanets(-1)),
            Action::Right => Transition::one(Command::AdjustCustomPlanets(1)),
            _ => Transition::none(),
        }
    }

    fn handle_ended(&mut self, _app: &mut App, action: Action) -> Transition {
        match action {
            Action::Confirm => Transition::one(Command::SetGameState(GameState::WaitingStart)),
            _ => Transition::none(),
        }
    }

    fn handle_in_game(&mut self, app: &mut App, action: Action) -> Transition {
        if matches!(action, Action::ToggleLog) {
            return Transition::one(Command::ToggleLog);
        }
        if matches!(action, Action::TogglePause) {
            return self.handle_toggle_pause(app);
        }

        match &mut app.ui.mode {
            UiMode::Normal => self.handle_normal(app, action),
            UiMode::MoveExplorer { .. } => self.handle_move_explorer(app, action),
        }
    }

    fn handle_normal(&mut self, app: &mut App, action: Action) -> Transition {
        match action {
            Action::Up => {
                self.move_focused_up(app);
                Transition::none()
            }
            Action::Down => {
                self.move_focused_down(app);
                Transition::none()
            }
            Action::Left => {
                app.ui.focus = Focus::Planets;
                app.ui.selectors.explorers.clear();
                app.ui
                    .selectors
                    .planets
                    .restore_last(app.planets_info.len());
                Transition::none()
            }
            Action::Right => {
                app.ui.focus = Focus::Explorers;
                app.ui.selectors.planets.clear();
                app.ui
                    .selectors
                    .explorers
                    .restore_last(app.explorers_info.len());
                Transition::none()
            }
            Action::SendAsteroid => {
                if let Some(planet_id) = app.ui.selectors.planets.last_selected_u32() {
                    return Transition::one(Command::QueueAsteroid { planet_id });
                }
                Transition::none()
            }
            Action::SendSunray => {
                if let Some(planet_id) = app.ui.selectors.planets.last_selected_u32() {
                    return Transition::one(Command::QueueSunray { planet_id });
                }
                Transition::none()
            }
            Action::MoveMode => {
                if app.ui.focus != Focus::Explorers {
                    return Transition::none();
                }

                if let Some(explorer_id) = app.ui.selectors.explorers.selected().map(|i| i as u32) {
                    let return_focus = app.ui.focus;
                    app.ui.focus = Focus::Planets;
                    app.ui
                        .selectors
                        .planets
                        .restore_last(app.planets_info.len());
                    app.ui.mode = UiMode::MoveExplorer {
                        explorer_id,
                        return_focus,
                    };
                    return Transition::one(Command::StopExplorerAI { explorer_id });
                } else {
                    app.ui.overlays.banner = None;
                }
                Transition::none()
            }
            _ => Transition::none(),
        }
    }

    fn handle_move_explorer(&mut self, app: &mut App, action: Action) -> Transition {
        match action {
            Action::Up => {
                app.ui.selectors.planets.move_up(app.planets_info.len());
                Transition::none()
            }
            Action::Down => {
                app.ui.selectors.planets.move_down(app.planets_info.len());
                Transition::none()
            }
            Action::Cancel => {
                let return_focus = match app.ui.mode {
                    UiMode::MoveExplorer { return_focus, .. } => return_focus,
                    _ => Focus::Explorers,
                };
                app.ui.mode = UiMode::Normal;
                app.ui.focus = return_focus;
                app.ui.selectors.planets.clear();
                app.ui.overlays.banner = None;
                Transition::none()
            }
            Action::Confirm => {
                let Some(explorer_id) = current_move_explorer_id(&app.ui.mode) else {
                    app.ui.mode = UiMode::Normal;
                    app.ui.overlays.banner = None;
                    return Transition::none();
                };
                let Some(planet_idx) = app.ui.selectors.planets.last_selected() else {
                    app.ui.overlays.banner = Some("Select planet first.".to_string());
                    return Transition::none();
                };
                let planet_id = planet_idx as u32;
                app.ui.mode = UiMode::Normal;
                app.ui.focus = Focus::Explorers;
                app.ui.selectors.planets.clear();
                app.ui.overlays.banner = None;
                Transition {
                    commands: vec![Command::MoveExplorer {
                        explorer_id,
                        planet_id,
                    }],
                }
            }
            _ => {
                app.ui.overlays.banner = None;
                Transition::none()
            }
        }
    }

    fn handle_toggle_pause(&mut self, app: &mut App) -> Transition {
        match app.get_game_state() {
            GameState::Running => Transition {
                commands: vec![Command::StopAll, Command::SetGameState(GameState::Paused)],
            },
            GameState::Paused => Transition {
                commands: vec![
                    Command::RestartAll,
                    Command::SetGameState(GameState::Running),
                ],
            },
            _ => Transition::none(),
        }
    }

    fn move_focused_up(&mut self, app: &mut App) {
        match app.ui.focus {
            Focus::Planets => app.ui.selectors.planets.move_up(app.planets_info.len()),
            Focus::Explorers => app.ui.selectors.explorers.move_up(app.explorers_info.len()),
        }
    }

    fn move_focused_down(&mut self, app: &mut App) {
        match app.ui.focus {
            Focus::Planets => app.ui.selectors.planets.move_down(app.planets_info.len()),
            Focus::Explorers => app
                .ui
                .selectors
                .explorers
                .move_down(app.explorers_info.len()),
        }
    }
}

fn current_move_explorer_id(mode: &UiMode) -> Option<u32> {
    match mode {
        UiMode::MoveExplorer { explorer_id, .. } => Some(*explorer_id),
        _ => None,
    }
}
