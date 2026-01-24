use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::app::App;
use crate::events::handle_events;
use crate::states::GameState;
use crate::ui::render_ui;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit {
            match self.get_game_state() {
                GameState::WaitingStart => self.waiting_loop(terminal)?,
                GameState::Running => self.active_loop(terminal)?,
                GameState::Paused => self.paused_loop(terminal)?,
            }
        }
        Ok(())
    }

    /// Loop dedicated exclusively to the initial waiting phase
    fn waiting_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        // Draw the start screen
        terminal
            .draw(|frame| render_ui(self, frame))
            .map_err(|_| "Error while drawing start screen")?;

        // Wait for user input (Start or Quit)
        handle_events(self)?;
        Ok(())
    }

    /// Loop: tick management and orchestrator
    fn active_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit && self.get_game_state() == GameState::Running {
            // 1. Always draw the current state (at 60+ FPS or as allowed by the terminal)
            terminal
                .draw(|frame| render_ui(self, frame))
                .map_err(|_| "Error while drawing UI")?;

            // 2. Calculate how long to wait for the next event
            // If a tick just passed, we wait until the next one
            // let _timeout = self
            //     .tick_rate
            //     .checked_sub(self.last_tick.elapsed())
            //     .unwrap_or(Duration::from_secs(0));

            // 3. Poll events (keyboard)
            handle_events(self)?;

            // 4. Poll the Library (Logic)
            // If 100ms have passed, we ask the library for the updated snapshot
            if self.last_tick.elapsed() >= self.tick_rate {
                // Call your library function
                self.planets = self.orchestrator.get_planet_states();
                self.orchestrator.send_asteroid_to_all()?;
                self.orchestrator.handle_game_messages()?;

                // Optional: if the game needs to advance, also call update
                // self.orchestrator.tick();

                self.last_tick = Instant::now();
            }
        }
        Ok(())
    }

    /// Pause loop: only consume UI messages, time frozen
    fn paused_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        // Draw the pause overlay
        terminal
            .draw(|frame| render_ui(self, frame))
            .map_err(|_| "Error while drawing pause screen")?;

        // Wait for user input
        handle_events(self)?;
        Ok(())
    }
}