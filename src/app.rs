use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use omc_galaxy::{Orchestrator, PlanetInfoMap, utils::ExplorerInfoMap};
use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{game_state::GameState, tui_loggers::LogBuffer};

use crate::ui_state::AppUi;

pub(crate) struct App {
    //State of the game
    pub(crate) gamestate: GameState,
    //Data about the game
    pub(crate) planets_info: PlanetInfoMap, //Planet Info
    pub(crate) explorers_info: ExplorerInfoMap,
    pub(crate) sunray_asteroid_ratio: u32,
    pub(crate) galaxy_topology: Vec<Vec<bool>>, // Esempio: ID pianeta -> Vicini
    pub(crate) incoming_sunray_asteroids_queue: VecDeque<(u32, bool)>,
    pub(crate) orchestrator: Orchestrator,

    //UI speed
    pub(crate) exit: bool,
    pub(crate) send_rate: Duration,
    pub(crate) last_tick: Instant,
    /// Elapsed simulation time (paused-aware).
    pub(crate) sim_elapsed: Duration,
    /// Last wall-clock sample used to advance `sim_elapsed`.
    pub(crate) sim_last_sample: Instant,
    // pub(crate) frame_rate: Duration, // Useful not to overload the CPU

    //Game logs
    pub log_entries: Arc<LogBuffer>,

    //UI state machine and selectors
    pub(crate) ui: AppUi,
}

impl App {
    pub(crate) fn new(
        mut orchestrator: Orchestrator,
        log_buffer: Arc<LogBuffer>,
    ) -> Result<Self, String> {
        Ok(Self {
            gamestate: GameState::WaitingStart,
            planets_info: orchestrator.get_planets_info(),
            explorers_info: orchestrator.get_explorer_states(),
            galaxy_topology: orchestrator.get_galaxy_topology(),
            incoming_sunray_asteroids_queue: VecDeque::new(),
            orchestrator,
            sunray_asteroid_ratio: 100,

            exit: false,
            last_tick: Instant::now(),
            send_rate: Duration::from_millis(1000),
            sim_elapsed: Duration::from_secs(0),
            sim_last_sample: Instant::now(),
            // frame_rate: Duration::from_millis(33), // UI fluida a 30 FPS
            log_entries: log_buffer,

            ui: AppUi::new(),
        })
    }

    /// Synchronizes the local App state with the latest data from the Orchestrator.
    /// This acts as a thread-safe snapshot for the UI.
    pub(crate) fn get_game_state(&self) -> GameState {
        self.gamestate.clone()
    }

    pub(crate) fn set_game_state(&mut self, state: GameState) {
        self.gamestate = state;
    }
    pub(crate) fn get_game_info(&mut self) -> Result<(), String> {
        self.planets_info = self.orchestrator.get_planets_info();
        self.explorers_info = self.orchestrator.get_explorer_states();
        self.galaxy_topology = self.orchestrator.get_galaxy_topology();
        Ok(())
    }
    pub(crate) fn get_game_info_without_explorers(&mut self) -> Result<(), String> {
        self.planets_info = self.orchestrator.get_planets_info();
        self.galaxy_topology = self.orchestrator.get_galaxy_topology();
        Ok(())
    }

    /// Advance simulation clock when running; freeze when paused/other.
    pub(crate) fn tick_sim_clock(&mut self, now: Instant) {
        if self.gamestate == GameState::Running {
            let delta = now.saturating_duration_since(self.sim_last_sample);
            self.sim_elapsed = self.sim_elapsed.saturating_add(delta);
        }
        self.sim_last_sample = now;
    }

    /// Reset simulation clock to zero.
    pub(crate) fn reset_sim_clock(&mut self, now: Instant) {
        self.sim_elapsed = Duration::from_secs(0);
        self.sim_last_sample = now;
    }

    /// Format elapsed simulation time as `hh:mm:ss`.
    pub(crate) fn sim_time_hms(&self) -> String {
        let total = self.sim_elapsed.as_secs();
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        let seconds = total % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
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

        self.get_game_info_without_explorers()?;
        Ok(())
    }
}

//Menù selector
impl App {
    /// Cambia la modalità selezionata (0 per Random, 1 per Custom)
    pub(crate) fn toggle_generation_mode(&mut self) {
        if self.ui.start.selected_mode == 0 {
            self.ui.start.selected_mode = 1;
        } else {
            self.ui.start.selected_mode = 0;
        }
    }

    /// Incrementa o decrementa il numero di pianeti per la modalità Custom
    /// Mantiene il valore in un range ragionevole (es. 1-50)
    pub(crate) fn adjust_custom_planets(&mut self, delta: i32) {
        let current = self.ui.start.custom_planet_count as i32;
        let new_value = (current + delta).clamp(1, 50);
        self.ui.start.custom_planet_count = new_value as u32;
    }
}
// Selector for the planet table
impl App {
    pub(crate) fn get_rocket_of_selected_planet(&self) -> String {
        match self.ui.selectors.planets.last_selected() {
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
        match self.ui.selectors.planets.last_selected() {
            Some(selected) => {
                let planet = self.planets_info.get_info(selected as u32).unwrap();
                format!(
                    "{} out of {}",
                    planet.charged_cells_count,
                    planet.energy_cells.len()
                )
            }
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_id_selected_planet(&self) -> String {
        match self.ui.selectors.planets.last_selected() {
            Some(selected) => selected.to_string(),
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_selected_planet(&self) -> Option<usize> {
        self.ui.selectors.planets.last_selected()
    }
    pub(crate) fn get_name_selected_planet(&self) -> String {
        if let Some(planet) = self.ui.selectors.planets.last_selected() {
            format!(
                "{:?}",
                self.planets_info.get_info(planet as u32).unwrap().name
            )
        } else {
            "None".to_string()
        }
    }
    pub(crate) fn get_supported_resource(&self, planet_id: u32) -> Vec<BasicResourceType> {
        let mut supported_resource_list: Vec<BasicResourceType> = Vec::new();
        if let Some(planet) = self.planets_info.get_info(planet_id) {
            if let Some(resources) = planet.supported_resources.clone() {
                for resource in resources {
                    supported_resource_list.push(resource);
                }
            }
        }
        supported_resource_list
    }
    pub(crate) fn get_supported_combination(&self, planet_id: u32) -> Vec<ComplexResourceType> {
        let mut supported_resource_list: Vec<ComplexResourceType> = Vec::new();
        if let Some(planet) = self.planets_info.get_info(planet_id) {
            if let Some(resources) = planet.supported_combination.clone() {
                for resource in resources {
                    supported_resource_list.push(resource);
                }
            }
        }
        supported_resource_list
    }
}

// Selector methods for explorers
impl App {
    pub(crate) fn get_planet_selected_explorer(&self) -> String {
        match self.ui.selectors.explorers.last_selected() {
            Some(selected) => match self.explorers_info.get_planet(&(selected as u32)) {
                Some(planet_id) => planet_id.to_string(),
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_id_selected_explorer(&self) -> String {
        match self.ui.selectors.explorers.last_selected() {
            Some(selected) => match self.explorers_info.get_id(&(selected as u32)) {
                Some(id) => id.to_string(),
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
}

// Handler sunray asteroid send
impl App {
    pub(crate) fn add_incoming_sunray_asteroid(&mut self) -> Result<(), String> {
        //select one random planet
        let planet_id = self.orchestrator.get_random_planet_id()?;
        //select sunray or asteroid based on the sunray/asteroid ratio
        let random_value = rand::random::<u32>() % 100;
        if random_value <= self.sunray_asteroid_ratio as u32 {
            self.add_incoming_sunray_for_planet(planet_id);
        } else {
            self.add_incoming_asteroid_for_planet(planet_id);
        }

        Ok(())
    }
    pub(crate) fn add_incoming_asteroid_for_planet(&mut self, planet_id: u32) {
        self.incoming_sunray_asteroids_queue
            .push_back((planet_id, false));
    }
    pub(crate) fn add_incoming_sunray_for_planet(&mut self, planet_id: u32) {
        self.incoming_sunray_asteroids_queue
            .push_back((planet_id, true));
    }

    pub(crate) fn pop_incoming_sunray_asteroid(&mut self) -> Option<(u32, bool)> {
        self.incoming_sunray_asteroids_queue.pop_front()
    }

    pub(crate) fn find_incoming_sunray_asteroid_for_planet(&self, planet_id: u32) -> Vec<bool> {
        self.incoming_sunray_asteroids_queue
            .iter()
            .filter(|(id, _)| *id == planet_id)
            .map(|(_, is_sunray)| *is_sunray)
            .collect()
    }
}

/// Formats a list of resources into a condensed string representation.
/// Example: "2.AP | 1.C" for 2 Carbon and 1 AI Partner.
pub(crate) fn bag_to_string(bag: &[common_game::components::resource::ResourceType]) -> String {
    use std::collections::HashMap;

    // Display order — defines both sort and symbol
    const RESOURCE_ORDER: &[&str] = &["AP", "R", "Do", "L", "W", "D", "S", "O", "C", "H"];

    let mut counts: HashMap<&str, u32> = RESOURCE_ORDER.iter().map(|k| (*k, 0)).collect();

    for resource in bag {
        let key = if resource.is_aipartner() {
            "AP"
        } else if resource.is_robot() {
            "R"
        } else if resource.is_dolphin() {
            "Do"
        } else if resource.is_life() {
            "L"
        } else if resource.is_water() {
            "W"
        } else if resource.is_oxygen() {
            "O"
        } else if resource.is_silicon() {
            "S"
        } else if resource.is_hydrogen() {
            "H"
        } else if resource.is_diamond() {
            "D"
        } else if resource.is_carbon() {
            "C"
        } else {
            continue; // unknown resource type — skip silently
        };
        *counts.entry(key).or_insert(0) += 1;
    }

    RESOURCE_ORDER
        .iter()
        .filter_map(|key| {
            let count = counts[*key];
            if count > 0 {
                Some(format!("{count}.{key}"))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("|")
}
