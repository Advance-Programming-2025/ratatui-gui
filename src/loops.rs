use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::app::App;
use crate::game_state::{GameState, handle_game_state};
use crate::ui::render_ui;

impl App {
    /// Start the game
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit {
            match self.get_game_state() {
                GameState::WaitingStart => self.waiting_loop(terminal)?,
                GameState::Running => self.active_loop(terminal)?,
                GameState::Paused => self.paused_loop(terminal)?,
                GameState::Ended => self.end_loop(terminal)?,
            }
        }
        Ok(())
    }

    ///Ending game loop
    fn end_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        // Draw the start screen
        terminal
            .draw(|frame| render_ui(self, frame))
            .map_err(|_| "Error while drawing start screen")?;

        // Wait for user input (Start or Quit)
        handle_game_state(self)?;
        Ok(())
    }

    /// Loop dedicated exclusively to the initial waiting phase
    fn waiting_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        // Draw the start screen
        terminal
            .draw(|frame| render_ui(self, frame))
            .map_err(|_| "Error while drawing start screen")?;

        // Wait for user input (Start or Quit)
        handle_game_state(self)?;
        Ok(())
    }

    /// Loop of actual game play
    fn active_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit && self.gamestate == GameState::Running {
            self.tick_sim_clock(Instant::now());
            // User Input
            handle_game_state(self)?;

            // Draw
            terminal
                .draw(|frame| render_ui(self, frame))
                .map_err(|_| "Error drawing UI")?;

            // Handling Messages
            // Process small batch at each iteration
            self.orchestrator.handle_game_messages()?;

            // Spatial event
            if self.last_tick.elapsed() >= self.send_rate {
                self.get_game_info()?; // Aggiorna info da orchestrator
                self.orchestrator.send_bag_content_request_from_ui()?;

                //Send sunray or asteroid 
                loop {
                    match self.pop_incoming_sunray_asteroid() {
                        Some((planet_id, is_sunray)) => {
                            self.orchestrator.send_celestial_from_gui(vec![planet_id], is_sunray)?;
                        }
                        None => break,
                    }
                }

                //Add asteroid or sunray to send
                if self.add_incoming_sunray_asteroid() == Err("No more planets alive".to_string()) {
                    self.gamestate = GameState::Ended;
                }

                self.last_tick = Instant::now();
            }

            // Small sleep to slow the loop update
            std::thread::sleep(Duration::from_millis(20));
        }
        Ok(())
    }

    /// Pause loop: only consume UI messages, time frozen
    fn paused_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit && self.gamestate == GameState::Paused {
            self.tick_sim_clock(Instant::now());
            // User Input
            handle_game_state(self)?;

            // Draw
            terminal
                .draw(|frame| render_ui(self, frame))
                .map_err(|_| "Error drawing UI")?;

            // Handling Messages
            // Process small batch at each iteration
            self.orchestrator.handle_game_messages()?;

            // Update snapshot
            self.get_game_info()?; // Aggiorna info da orchestrator

            // Sleep
            std::thread::sleep(Duration::from_millis(1));
        }

        Ok(())
    }
}
