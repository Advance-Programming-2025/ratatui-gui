use omc_galaxy::{Orchestrator, PlanetInfoMap, utils::ExplorerInfoMap};
use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{game_state::GameState, tui_loggers::LogBuffer};
use omc_galaxy::settings;

use crate::selector::Selector;

pub struct App {
    //State of the game
    pub(crate) gamestate: GameState,
    //Data about the game
    pub(crate) planets_info: PlanetInfoMap, //Planet Info
    pub(crate) explorers_info: ExplorerInfoMap,
    pub(crate) sunray_rate: u32,
    pub(crate) galaxy_topology: Vec<Vec<bool>>, // Esempio: ID pianeta -> Vicini
    pub(crate) incoming_sunray_asteroids_queue: VecDeque<(u32, bool)>,
    pub(crate) orchestrator: Orchestrator,

    //UI speed
    pub(crate) exit: bool,
    pub(crate) send_rate: Duration,
    pub(crate) last_tick: Instant,
    // pub(crate) frame_rate: Duration, // Useful not to overload the CPU

    //Game logs
    pub log_entries: Arc<LogBuffer>,

    //UI planet selector variables
    pub(crate) general_selector: Selector,
    // pub(crate) planet_selector: TableState,
    // pub(crate) explorer_selector: TableState,

    //UI log overlay toggle
    pub show_log_overlay: bool,
    pub planet_typed: Option<u32>,

    /// Tracks the currently highlighted menu option on the start screen.
    /// 0 for Random, 1 for Custom.
    pub(crate) selected_mode: u8,

    /// Stores the user-defined number of planets for Custom mode.
    pub(crate) custom_planet_count: u32,
}

impl App {
    pub fn new(mut orchestrator: Orchestrator, log_buffer: Arc<LogBuffer>) -> Result<Self, String> {
        Ok(Self {
            gamestate: GameState::WaitingStart,
            planets_info:  orchestrator.get_planets_info(),
            explorers_info: orchestrator.get_explorer_states(),
            galaxy_topology: orchestrator.get_galaxy_topology(),
            incoming_sunray_asteroids_queue: VecDeque::new(),
            orchestrator,
            sunray_rate: settings::get_sunray_probability(),

            exit: false,
            last_tick: Instant::now(),
            send_rate: Duration::from_millis(1000),
            // frame_rate: Duration::from_millis(33), // UI fluida a 30 FPS
            log_entries: log_buffer,

            general_selector: Selector::new(0, 0),
            // planet_selector: TableState::default(),
            // explorer_selector: TableState::default(),

            show_log_overlay: false,
            planet_typed: None,

            selected_mode: 0,
            custom_planet_count: 0,
        })
    }

    /// Synchronizes the local App state with the latest data from the Orchestrator.
    /// This acts as a thread-safe snapshot for the UI.
    pub fn get_game_state(&self) -> GameState {
        self.gamestate.clone()
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.gamestate = state;
    }
    pub(crate) fn get_game_info(&mut self) -> Result<(), String> {
        self.planets_info = self.orchestrator.get_planets_info();
        self.explorers_info = self.orchestrator.get_explorer_states();
        self.sunray_rate = settings::get_sunray_probability();
        self.galaxy_topology = self.orchestrator.get_galaxy_topology();
        Ok(())
    }
    pub(crate) fn get_game_info_without_explorers(&mut self) -> Result<(), String> {
        self.planets_info = self.orchestrator.get_planets_info();
        self.sunray_rate = settings::get_sunray_probability();
        self.galaxy_topology = self.orchestrator.get_galaxy_topology();
        Ok(())
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

    pub(crate) fn set_sunray_increment(&mut self) {
        settings::set_sunray_probability(self.sunray_rate + 5);
    }
    pub(crate) fn set_sunray_decrement(&mut self) {
        settings::set_sunray_probability(self.sunray_rate - 5);
    }
}

//Menù selector
impl App {
    /// Cambia la modalità selezionata (0 per Random, 1 per Custom)
    pub fn toggle_generation_mode(&mut self) {
        if self.selected_mode == 0 {
            self.selected_mode = 1;
        } else {
            self.selected_mode = 0;
        }
    }

    /// Incrementa o decrementa il numero di pianeti per la modalità Custom
    /// Mantiene il valore in un range ragionevole (es. 1-50)
    pub fn adjust_custom_planets(&mut self, delta: i32) {
        let current = self.custom_planet_count as i32;
        let new_value = (current + delta).clamp(1, 50);
        self.custom_planet_count = new_value as u32;
    }
}
// Selector for the planet table
impl App {
    pub(crate) fn get_rocket_of_selected_planet(&self) -> String {
        match self.general_selector.get_last_planet_selected() {
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
        match self.general_selector.get_last_planet_selected(){
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
        match self.general_selector.get_last_planet_selected() {
            Some(selected) => selected.to_string(),
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_name_selected_planet(&self) -> String {
        if let Some(planet) = self.general_selector.get_last_planet_selected() {
            format!(
                "{:?}",
                self.planets_info.get_info(planet as u32).unwrap().name
            )
        } else {
            "None".to_string()
        }
    }
}

// Selector methods for explorers
impl App {
    
    pub(crate) fn get_bag_selected_explorer(&self) -> String {
        match self.general_selector.get_last_explorer_selected(){
            Some(selected) => match self.explorers_info.get_bag(&(selected as u32)) {
                Some(bag) => bag_to_string(bag),
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_planet_selected_explorer(&self) -> String {
        match self.general_selector.get_last_explorer_selected() {
            Some(selected) => match self.explorers_info.get_planet(&(selected as u32)) {
                Some(planet_id) => planet_id.to_string(),
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_id_selected_explorer(&self) -> String {
        match self.general_selector.get_last_explorer_selected() {
            Some(selected) => match self.explorers_info.get_id(&(selected as u32)) {
                Some(id) => id.to_string(),
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
}

// Methods for handling both explorer and planet
impl App {}

// Handler sunray asteroid send
impl App {
    pub(crate) fn add_incoming_sunray_asteroid(&mut self) -> Result<(), String> {
        let planet_id = self.orchestrator.get_random_planet_id()?;
        let is_sunray = settings::does_sunray_spawn();
        self.incoming_sunray_asteroids_queue
            .push_back((planet_id, is_sunray));

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
    const RESOURCE_ORDER: &[&str] = &["AP", "R", "Do", "L", "W", "O", "S", "H", "D", "C"];

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
