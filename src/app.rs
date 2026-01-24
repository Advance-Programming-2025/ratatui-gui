use omc_galaxy::{ExplorerStatusNotLock, Orchestrator, PlanetStatusNotLock};
use std::time::{Duration, Instant};

use crate::states::GameState;

pub struct App {
    pub(crate) gamestate: GameState,
    pub(crate) orchestrator: Orchestrator,
    pub(crate) planets: PlanetStatusNotLock,
    pub(crate) explorers: ExplorerStatusNotLock,
    pub(crate) exit: bool,
    pub(crate) tick_rate: Duration,
    pub(crate) last_tick: Instant,
}

impl App {
    pub fn new(orchestrator: Orchestrator) -> Self {
        Self {
            gamestate: GameState::WaitingStart,
            planets: orchestrator.get_planet_states(),
            explorers: orchestrator.get_explorer_states(),
            orchestrator,
            exit: false,
            tick_rate: Duration::from_millis(100),
            last_tick: Instant::now(),
        }
    }

    pub fn get_game_state(&self) -> GameState {
        self.gamestate.clone()
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.gamestate = state;
    }

    pub fn initialize_by_file(&mut self) -> Result<(), String> {
        // Load env
        dotenv::dotenv().ok();
        
        // Give the absolute path for the init file
        let file_path =
            std::env::var("INPUT_FILE").map_err(|_| "Set INPUT_FILE in .env or env vars")?;

        self.orchestrator
            .initialize_galaxy_by_file(file_path.as_str().trim())
            .map_err(|_| "Failed to initialize galaxy")?;
        Ok(())
    }
}