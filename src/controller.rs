//! UI controller state machine.
//! Dispatches actions using nested (phase, mode, focus) state.

use common_game::components::resource::ResourceType::{Basic, Complex};

use crate::{
    app::App,
    commands::Command,
    game_state::GameState,
    input::Action,
    ui_state::{Focus, UiMode},
};

/// Result of one controller step.
pub(crate) struct Transition {
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
pub(crate) struct Controller;

impl Controller {
    /// Handle one semantic action.
    pub(crate) fn handle(&mut self, app: &mut App, action: Action) -> Transition {
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
            UiMode::GenerateResource { .. } => self.handle_generate_resource(app, action),
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
                    app.ui.overlays.banner = Some(
                        "Select a planet with ↑/↓, Enter to confirm, Esc to cancel.".to_string(),
                    );
                    app.ui.mode = UiMode::MoveExplorer {
                        explorer_id,
                        return_focus,
                    };
                    return Transition::one(Command::StopExplorerAI { explorer_id });
                } else {
                    app.ui.overlays.banner = Some("Select explorer first.".to_string());
                }
                Transition::none()
            }
            Action::GenerateResource => {
                if app.ui.focus != Focus::Explorers {
                    return Transition::none();
                }
                app.ui.selectors.basic_resources.clear();
                app.ui.selectors.complex_resource.clear();

                if let Some(explorer_id) = app.ui.selectors.explorers.selected().map(|i| i as u32) {
                    // 1. Recupera il pianeta corrente su cui si trova questo specifico Explorer
                    let planet_id = match app.explorers_info.get(&explorer_id) {
                        Some(explorer) => explorer.current_planet_id,
                        None => {
                            app.ui.overlays.banner = Some("Impossibile determinare la posizione dell'explorer.".to_string());
                            return Transition::none();
                        }
                    };

                    let return_focus = app.ui.focus;
                    
                    // 2. Passa il PLANET_ID (e non l'explorer_id) al metodo!
                    let available_basic = app.get_supported_resource(planet_id);
                    let available_complex = app.get_supported_combination(planet_id); // Se usi questo per le complesse

                    // 3. Inizializza le liste originali dei selettori con i dati reali del pianeta
                    app.ui
                        .selectors
                        .basic_resources
                        .set_original_list(available_basic.clone());
                    app.ui
                        .selectors
                        .complex_resource
                        .set_original_list(available_complex.clone());

                    // 4. Ripristina gli indici visivi della selezione interna
                    app.ui
                        .selectors
                        .basic_resources
                        .restore_last(available_basic.len());

                    app.ui.mode = UiMode::GenerateResource {
                        explorer_id,
                        return_focus,
                    };
                    return Transition::one(Command::StopExplorerAI { explorer_id });
                } else {
                    app.ui.overlays.banner = Some("Select explorer first.".to_string());
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
                let Some((explorer_id, return_focus)) = current_move_explorer_context(&app.ui.mode)
                else {
                    app.ui.mode = UiMode::Normal;
                    app.ui.overlays.banner = None;
                    return Transition::none();
                };
                let Some(planet_idx) = app.ui.selectors.planets.selected() else {
                    app.ui.overlays.banner = Some("Select planet first.".to_string());
                    return Transition::none();
                };
                let planet_id = planet_idx as u32;
                app.ui.mode = UiMode::Normal;
                app.ui.focus = return_focus;
                app.ui.selectors.planets.clear();
                app.ui.overlays.banner = None;
                Transition {
                    commands: vec![Command::MoveExplorer {
                        explorer_id,
                        planet_id,
                    }],
                }
            }
            _ => Transition::none(),
        }
    }

    fn handle_generate_resource(&mut self, app: &mut App, action: Action) -> Transition {
        match action {
            Action::Up => {
                app.ui
                    .selectors
                    .basic_resources
                    .move_up(app.ui.selectors.basic_resources.len());
                Transition::none()
            }
            Action::Down => {
                app.ui
                    .selectors
                    .basic_resources
                    .move_down(app.ui.selectors.basic_resources.len());
                Transition::none()
            }
            Action::Cancel => {
                let return_focus = match app.ui.mode {
                    UiMode::GenerateResource { return_focus, .. } => return_focus,
                    _ => Focus::Explorers,
                };
                app.ui.mode = UiMode::Normal;
                app.ui.focus = return_focus;
                app.ui.selectors.basic_resources.clear();
                app.ui.overlays.banner = None;
                Transition::none()
            }
            Action::Confirm => {
                // 1. Recupera l'explorer_id CORRETTO dallo stato della modalità UI attuale
                let Some((explorer_id, return_focus)) =
                    current_generate_resource_context(&app.ui.mode)
                else {
                    app.ui.mode = UiMode::Normal;
                    app.ui.overlays.banner = None;
                    return Transition::none();
                };

                // 2. Controlla quale widget ha una selezione ATTIVA (usando selected(), non last_selected())
                let basic_idx = app.ui.selectors.basic_resources.state_mut().selected();
                let complex_idx = app.ui.selectors.complex_resource.state_mut().selected();

                // 3. Determina la risorsa basandoti su cosa è effettivamente evidenziato a schermo
                let resource = match (basic_idx, complex_idx) {
                    (Some(index), _) => {
                        // Se c'è una selezione sulla lista basic, diamo la priorità o gestiamo questa
                        let basic = app
                            .ui
                            .selectors
                            .basic_resources
                            .get_element_from_original_list(index);
                        Basic(basic)
                    }
                    (None, Some(index)) => {
                        let complex = app
                            .ui
                            .selectors
                            .complex_resource
                            .get_element_from_original_list(index);
                        Complex(complex)
                    }
                    (None, None) => {
                        app.ui.overlays.banner =
                            Some("Seleziona una risorsa prima di confermare!".to_string());
                        return Transition::none();
                    }
                };

                // 4. Ripristina lo stato normale della UI
                app.ui.mode = UiMode::Normal;
                app.ui.focus = return_focus;
                app.ui.selectors.basic_resources.clear();
                app.ui.selectors.complex_resource.clear();
                app.ui.overlays.banner = None;

                // 5. Invia il comando con l'explorer_id garantito
                Transition::one(Command::GenerateResource {
                    explorer_id,
                    resource,
                })
            }
            _ => Transition::none(),
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

fn current_generate_resource_explorer_id(mode: &UiMode) -> Option<u32> {
    match mode {
        UiMode::GenerateResource { explorer_id, .. } => Some(*explorer_id),
        _ => None,
    }
}

fn current_move_explorer_context(mode: &UiMode) -> Option<(u32, Focus)> {
    match mode {
        UiMode::MoveExplorer {
            explorer_id,
            return_focus,
        } => Some((*explorer_id, *return_focus)),
        _ => None,
    }
}

fn current_generate_resource_context(mode: &UiMode) -> Option<(u32, Focus)> {
    match mode {
        UiMode::GenerateResource {
            explorer_id,
            return_focus,
        } => Some((*explorer_id, *return_focus)),
        _ => None,
    }
}
