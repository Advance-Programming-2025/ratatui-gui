use omc_galaxy::{Orchestrator, PlanetInfoMap, utils::ExplorerInfoMap};
use ratatui::widgets::TableState;
use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{game_state::GameState, tui_loggers::LogBuffer};
use omc_galaxy::settings;

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
    pub(crate) frame_rate: Duration, // Useful not to overload the CPU

    //Game logs
    pub log_entries: Arc<LogBuffer>,

    //UI planet selector variables
    pub(crate) planet_selector: TableState,
    pub(crate) explorer_selector: TableState,

    //UI log overlay toggle
    pub show_log_overlay: bool,
}

impl App {
    pub fn new(orchestrator: Orchestrator, log_buffer: Arc<LogBuffer>) -> Result<Self, String> {
        Ok(Self {
            gamestate: GameState::WaitingStart,
            planets_info: orchestrator.get_planets_info(),
            explorers_info: orchestrator.get_explorer_states(),
            galaxy_topology: orchestrator.get_galaxy_topology(),
            incoming_sunray_asteroids_queue: VecDeque::new(),
            orchestrator,
            sunray_rate: settings::get_sunray_probability(),

            exit: false,
            last_tick: Instant::now(),
            send_rate: Duration::from_millis(1000),
            frame_rate: Duration::from_millis(33), // UI fluida a 30 FPS
            log_entries: log_buffer,

            planet_selector: TableState::default(),
            explorer_selector: TableState::default(),

            show_log_overlay: false,
        })
    }

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

// Selector for the planet table
impl App {
    pub(crate) fn enable_planet_selector(&mut self) -> bool {
        if self.planets_info.len() > 0 {
            self.planet_selector.select(Some(0));
            true
        } else {
            false
        }
    }
    pub(crate) fn increment_planet_selector(&mut self) {
        let n = self.planets_info.len();
        if n == 0 {
            return;
        }

        let i = match self.planet_selector.selected() {
            Some(i) => {
                if i >= n - 1 {
                    n - 1
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.planet_selector.select(Some(i));
    }

    pub(crate) fn decrement_planet_selector(&mut self) {
        let n = self.planets_info.len();
        if n == 0 {
            return;
        }

        let i = match self.planet_selector.selected() {
            Some(i) => {
                if i == 0 {
                    if self.enable_explorer_selector() {
                        self.disable_planet_selector();
                    }
                    n
                } else {
                    i - 1
                }
            }
            None => n - 1,
        };
        if i != n {
            self.planet_selector.select(Some(i));
        }
    }

    pub(crate) fn disable_planet_selector(&mut self) {
        self.planet_selector.select(None);
    }

    pub(crate) fn get_rocket_of_selected_planet(&self) -> String {
        match self.planet_selector.selected() {
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
        match self.planet_selector.selected() {
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
        match self.planet_selector.selected() {
            Some(selected) => selected.to_string(),
            None => format!("None"),
        }
    }
    pub(crate) fn get_name_selected_planet(&self) -> String {
        if let Some(planet) = self.planet_selector.selected() {
            format!(
                "{:?}",
                self.planets_info.get_info(planet as u32).unwrap().name
            )
        } else {
            "None".to_string()
        }
    }

    // pub(crate) fn get_incoming_sunray_asteroids_selected_planet(&self)->String{
    //     if let Some(planet) = self.planet_selector.selected() {
    //         format!(
    //             "{}",
    //             match self.incoming_sunray_asteroids.get(&(planet as u32)){
    //                 Some(value)=>value.to_string(),
    //                 None=>planet.to_string()
    //             }
    //         )
    //     } else {
    //         "None".to_string()
    //     }
    // }
}
// Selector for the explorer table
impl App {
    pub(crate) fn enable_explorer_selector(&mut self) -> bool {
        if self.explorers_info.len() > 0 {
            self.explorer_selector
                .select(Some(self.explorers_info.len() - 1));
            true
        } else {
            false
        }
    }
    pub(crate) fn increment_explorer_selector(&mut self) {
        let n = self.explorers_info.len();
        if n == 0 {
            return;
        }

        let i = match self.explorer_selector.selected() {
            Some(i) => {
                if i >= n - 1 {
                    if self.enable_planet_selector() {
                        self.disable_explorer_selector();
                    }
                    n
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        if i != n {
            self.explorer_selector.select(Some(i));
        }
    }

    pub(crate) fn decrement_explorer_selector(&mut self) {
        let n = self.explorers_info.len();
        if n == 0 {
            return;
        }

        let i = match self.explorer_selector.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => n - 1,
        };

        self.explorer_selector.select(Some(i));
    }

    pub(crate) fn disable_explorer_selector(&mut self) {
        self.explorer_selector.select(None);
    }
    pub(crate) fn get_bag_selected_explorer(&self) -> String {
        match self.explorer_selector.selected() {
            Some(selected) => match self.explorers_info.get_bag(&(selected as u32)) {
                Some(bag) => {
                    let mut bag_content = String::new();
                    for cell in bag {
                        bag_content.push_str(&format!("{:?} ", cell));
                    }
                    bag_content
                }
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_planet_selected_explorer(&self) -> String {
        match self.explorer_selector.selected() {
            Some(selected) => match self.explorers_info.get_planet(&(selected as u32)) {
                Some(planet_id) => format!("{}", planet_id),
                None => "None".to_string(),
            },
            None => "None".to_string(),
        }
    }
    pub(crate) fn get_id_selected_explorer(&self) -> String {
        match self.explorer_selector.selected() {
            Some(selected) => match self.explorers_info.get_id(&(selected as u32)) {
                Some(id) => format!("{}", id),
                None => "None".to_string(),
            },
            None => format!("None"),
        }
    }
}

// Handler sunray asteroid send
impl App {
    pub(crate) fn add_incoming_sunray_asteroid(&mut self) -> Result<(), String> {
        let planet_id = self.orchestrator.get_random_planet_id()?;
        let is_sunray = settings::does_sunray_spawn();
        self.incoming_sunray_asteroids_queue
            .push_back((planet_id, is_sunray));

        Ok(())
    }

    pub(crate) fn pop_incoming_sunray_asteroid(&mut self) -> Option<(u32, bool)> {
        self.incoming_sunray_asteroids_queue.pop_front()
    }

    pub(crate) fn find_incoming_sunray_asteroid_for_planet(&self, planet_id: u32) -> Vec<bool> {
        let mut vec = Vec::new();
        for (id, is_sunray) in &self.incoming_sunray_asteroids_queue {
            if planet_id == *id {
                vec.push(*is_sunray);
            }
        }
        vec
    }
}
