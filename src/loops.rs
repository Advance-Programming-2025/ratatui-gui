use omc_galaxy::settings::{self};
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::app::App;
use crate::game_state::{GameState, handle_game_state};
use crate::ui::render_ui;

impl App {
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

    /// Loop: tick management and orchestrator
    fn active_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit && self.gamestate == GameState::Running {
            // --- 1. INPUT UTENTE (PRIMA DI TUTTO per massima reattività) ---
            handle_game_state(self)?;

            // --- 2. DISEGNO (Solo se è passato il tempo del frame_rate) ---
            terminal
                .draw(|frame| render_ui(self, frame))
                .map_err(|_| "Error drawing UI")?;

            // --- 3. GESTIONE MESSAGGI (Continua) ---
            // Processiamo piccoli batch ad ogni iterazione del loop
            self.orchestrator.handle_game_messages()?;

            // --- 4. TICK LOGICA (Eventi Spaziali) ---
            if self.last_tick.elapsed() >= self.send_rate {
                
                self.get_game_info();

                //Invia o sunray o asteroid in base alla code definita in App
                match self.pop_incoming_sunray_asteroid(){
                    Some((planet_id, true))=>self.orchestrator.send_sunray_from_gui(vec![planet_id])?,
                    Some((planet_id, false))=>self.orchestrator.send_asteroid_from_gui(vec![planet_id])?,
                    None=>{},
                }
                //Questa funzione ritorna un errore se non ci sono più pianeti vivi
                // if self.orchestrator.send_sunray_or_asteroid()
                //     == Err("No more planets alive".to_string())
                // {
                //     self.gamestate = GameState::Ended;
                // }

                //Aggiungi un asteroid o sunray da inviare
                if self.add_incoming_sunray_asteroid()== Err("No more planets alive".to_string()){
                    self.gamestate = GameState::Ended;
                }
            
                self.last_tick = Instant::now();

            }

            // --- 5. RIPOSO (Opzionale ma consigliato) ---
            // Un piccolo sleep per non bruciare la CPU se il loop è troppo veloce
            std::thread::sleep(Duration::from_millis(1));
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
        handle_game_state(self)?;
        Ok(())
    }
}
