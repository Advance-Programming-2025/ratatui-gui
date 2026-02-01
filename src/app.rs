use omc_galaxy::{Orchestrator, PlanetInfoMap, utils::ExplorerInfoMap};
use ratatui::widgets::TableState;
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
    pub(crate) galaxy_topology: Vec<Vec<bool>>, // Esempio: ID pianeta -> Vicini

    //UI speed
    pub(crate) exit: bool,
    pub(crate) tick_rate: Duration,
    pub(crate) last_tick: Instant,
    pub(crate) frame_rate: Duration, // Useful not to overload the CPU

    //Game logs
    pub log_entries: Arc<LogBuffer>,

    //UI planet selector variables
    pub(crate) table_state: TableState,

    //UI log overlay toggle
    pub show_log_overlay: bool,
}

impl App {
    pub fn new(orchestrator: Orchestrator, log_buffer: Arc<LogBuffer>) -> Self {
        Self {
            gamestate: GameState::WaitingStart,
            planets_info: orchestrator.get_planets_info(),
            explorers_info: orchestrator.get_explorer_states(),
            galaxy_topology: orchestrator.get_galaxy_topology(),
            orchestrator,
            probability_sunray: settings::get_sunray_probability(),

            exit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(500),
            frame_rate: Duration::from_millis(33), // UI fluida a 30 FPS
            log_entries: log_buffer,

            table_state: TableState::default(),

            show_log_overlay: false,
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
        let n = self.planets_info.len();
        if n == 0 {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= n - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.table_state.select(Some(i));
    }

    pub(crate) fn decrement_id_selector(&mut self) {
        let n = self.planets_info.len();
        if n == 0 {
            return;
        }

        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    n - 1
                } else {
                    i - 1
                }
            }
            None => n - 1,
        };

        self.table_state.select(Some(i));
    }
    pub(crate) fn get_rocket_of_selected_planet(&self) -> String {
        match self.table_state.selected() {
            Some(selected) => {
                if self.planets_info.get_info(selected as u32).unwrap().rocket {
                    "AVAILABLE".to_string()
                } else {
                    "NOT PRESENT".to_string()
                }
            }
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_cells_info_selected_planet(&self) -> String {
        match self.table_state.selected() {
            Some(selected) => {
                let planet = self.planets_info.get_info(selected as u32).unwrap();
                format!(
                    "{} out of {}",
                    planet.charged_cells_count,
                    planet.energy_cells.len()
                )
            }
            None => format!("None"),
        }
    }
    pub(crate) fn get_id_selected_planet(&self) -> String {
        match self.table_state.selected() {
            Some(selected) => selected.to_string(),
            None => format!("None"),
        }
    }
    pub(crate) fn get_name_selected_planet(&self) -> String {
        if let Some(planet) = self.table_state.selected() {
            format!(
                "{:?}",
                self.planets_info.get_info(planet as u32).unwrap().name
            )
        } else {
            "None".to_string()
        }
    }
}
