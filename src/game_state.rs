use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::{time::Duration};

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

/// Global input handler that bridges TUI events with game logic.
/// Manages entity selection, galaxy navigation, and orchestrator commands.
pub fn handle_game_state(app: &mut App) -> Result<(), String> {
    // Very short timeout for responsive input
    if event::poll(Duration::from_millis(5)).map_err(|_| "Polling error")? {
        if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
            if key.kind == event::KeyEventKind::Press {
                match (key.code, app.get_game_state()) {
                    // Global events - respond immediately on key press
                    (KeyCode::Char('q'), _) => {
                        app.exit = true;
                    }
                    (KeyCode::Enter, GameState::WaitingStart) => {
                        let mattia_explorers = vec![(0, 0)];
                        let tommy_explorers = vec![(1, 1)];
                        app.orchestrator
                            .start_all(&mattia_explorers, &tommy_explorers)?;
                        app.set_game_state(GameState::Running);
                    }
                    (KeyCode::Char('p'), GameState::Running) => {
                        app.orchestrator.stop_all()?;
                        app.set_game_state(GameState::Paused);
                    }
                    (KeyCode::Char('p'), GameState::Paused) => {
                        app.orchestrator.restart_all()?;
                        app.set_game_state(GameState::Running);
                    }
                    (KeyCode::Char('o'), GameState::Paused) => app.set_sunray_increment(),
                    (KeyCode::Char('i'), GameState::Paused) => app.set_sunray_decrement(),

                    (KeyCode::Right, GameState::WaitingStart) => {
                        if app.selected_mode == 1 { // Solo se siamo in Custom
                            app.adjust_custom_planets(1);
                        }
                    }
                    
                    (KeyCode::Left, GameState::WaitingStart) => {
                        if app.selected_mode == 1 { // Solo se siamo in Custom
                            app.adjust_custom_planets(-1);
                        }
                    }

                    (KeyCode::Up, GameState::WaitingStart)|(KeyCode::Down, GameState::WaitingStart)=>{
                        app.toggle_generation_mode();
                    }
                    // Navigation events
                    (KeyCode::Up, GameState::Running) | (KeyCode::Up, GameState::Paused)=> {
                        match (
                            app.explorer_selector.selected(),
                            app.planet_selector.selected(),
                        ) {
                            (Some(_), None) => app.decrement_explorer_selector(),
                            (None, Some(_)) => app.decrement_planet_selector(),
                            (None, None) => app.decrement_planet_selector(),
                            _ => {}
                        }
                    }
                    (KeyCode::Down, GameState::Running)|(KeyCode::Down, GameState::Paused) => {
                        match (
                            app.explorer_selector.selected(),
                            app.planet_selector.selected(),
                        ) {
                            (Some(_), None) => app.increment_explorer_selector(),
                            (None, Some(_)) => app.increment_planet_selector(),
                            (None, None) => app.increment_explorer_selector(),
                            _ => {}
                        }
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
                    // Send asteroid with 'A' (for testing)
                    (KeyCode::Char('a'), GameState::Running)
                    | (KeyCode::Char('a'), GameState::Paused) => {
                        match app.planet_selector.selected() {
                            Some(planet_id) => {
                                app.add_incoming_asteroid_for_planet(planet_id as u32)
                            }
                            None => {}
                        }
                    }
                    // Send sunray with 'S' (for testing)
                    (KeyCode::Char('s'), GameState::Running)
                    | (KeyCode::Char('s'), GameState::Paused) => {
                        match app.planet_selector.selected() {
                            Some(planet_id) => app.add_incoming_sunray_for_planet(planet_id as u32),
                            None => {}
                        }
                    }
                    // Move the explorer to another planet with selecting the explorer and then typing the planet id and press enter
                    (KeyCode::Enter, GameState::Running) | (KeyCode::Enter, GameState::Paused) => {
                        if let Some(explorer_id) = app.explorer_selector.selected() {
                            //type the planet id in the terminal
                            if let Some(planet_id) = app.planet_typed {
                                if app.planets_info.contains(&planet_id) {
                                    app.orchestrator.send_move_explorer_from_gui(
                                        explorer_id as u32,
                                        planet_id,
                                    )?;
                                    app.planet_typed = None;
                                } else {
                                    app.planet_typed = None;
                                }
                            }
                        } else {
                            app.planet_typed = None;
                        }
                    }
                    (KeyCode::Char(d), _) if d.is_digit(10) => {
                        let digit = d.to_digit(10).unwrap();
                        app.planet_typed = match app.planet_typed {
                            Some(num) => {
                                // Prevent overflow before multiplication
                                if num > (u32::MAX - digit) / 10 {
                                    None
                                } else {
                                    Some(num * 10 + digit)
                                }
                            }
                            None => Some(digit),
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
