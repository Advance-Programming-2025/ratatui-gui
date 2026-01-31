use omc_galaxy::{
    Orchestrator, PlanetInfoMap,
    utils::{ExplorerInfo, ExplorerInfoMap, registry::PlanetTypeIter},
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{game_state::GameState, tui_loggers::LogBuffer};
use omc_galaxy::settings;

pub struct App {
    //State of the game
    pub(crate) gamestate: GameState,
    //Data about the game
    pub(crate) orchestrator: Orchestrator,
    pub(crate) planets_info: PlanetInfoMap, //Planet Info
    pub(crate) explorers_info: ExplorerInfoMap,
    pub(crate) probability_sunray: u32,
    // pub(crate) adjacency_list: HashMap<u32, Vec<u32>>, // Esempio: ID pianeta -> Vicini

    //UI speed
    pub(crate) exit: bool,
    pub(crate) tick_rate: Duration,
    pub(crate) last_tick: Instant,
    pub(crate) frame_rate: Duration, // Useful not to overload the CPU

    //Game logs
    pub log_entries: Arc<LogBuffer>,

    //UI planet selector variables
    pub planet_id_selector: Option<u32>,
}

impl App {
    pub fn new(orchestrator: Orchestrator, log_buffer: Arc<LogBuffer>) -> Self {
        Self {
            gamestate: GameState::WaitingStart,
            planets_info: orchestrator.get_planets_info(),
            explorers_info: orchestrator.get_explorer_states(),
            orchestrator,
            probability_sunray: settings::get_sunray_probability(),

            exit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(200),
            frame_rate: Duration::from_millis(33), // UI fluida a 30 FPS
            log_entries: log_buffer,

            planet_id_selector: None,
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

    pub(crate) fn set_sunray_increment(&mut self) {
        settings::set_sunray_probability(self.probability_sunray + 1);
    }
    pub(crate) fn set_sunray_decrement(&mut self) {
        settings::set_sunray_probability(self.probability_sunray - 1);
    }
}

// Selector for the planet table
impl App {
    pub(crate) fn increment_id_selector(&mut self) {
        self.planet_id_selector = match self.planet_id_selector {
            Some(id) => {
                let new_id = if id == (self.planets_info.len() - 1) as u32 {
                    0
                } else {
                    id + 1
                };
                Some(new_id)
            }
            None => Some(0),
        }
    }

    pub(crate) fn decrement_id_selector(&mut self) {
        self.planet_id_selector = match self.planet_id_selector {
            Some(id) => {
                let new_id = if id == 0 as u32 {
                    (self.planets_info.len() - 1) as u32
                } else {
                    id - 1
                };
                Some(new_id)
            }
            None => Some((self.planets_info.len() - 1) as u32),
        }
    }
}
