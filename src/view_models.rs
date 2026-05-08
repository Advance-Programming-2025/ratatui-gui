use omc_galaxy::Status;

use crate::app::{bag_to_string, App};

#[derive(Debug, Clone)]
pub struct ExplorerRowVm {
    pub id: u32,
    pub status: &'static str,
    pub bag: String,
    pub planet_id: String,
}

#[derive(Debug, Clone)]
pub struct PlanetRowVm {
    pub id: u32,
    pub status: String,
    pub rocket: String,
    pub energy: String,
    pub incoming: String,
    pub highlight_neighbor: bool,
}

pub fn explorer_rows(app: &App) -> Vec<ExplorerRowVm> {
    app.explorers_info
        .iter()
        .map(|(id, info)| ExplorerRowVm {
            id: *id,
            status: status_text(info.status),
            bag: bag_to_string(&info.bag),
            planet_id: info.current_planet_id.to_string(),
        })
        .collect()
}

pub fn planet_rows(app: &App) -> Vec<PlanetRowVm> {
    app.planets_info
        .iter()
        .map(|(id, info)| {
            let charged = info.charged_cells_count;
            let total = info.energy_cells.len();
            let energy = energy_bar(charged, total);

            let incoming: String = app
                .find_incoming_sunray_asteroid_for_planet(*id)
                .iter()
                .map(|&is_sunray| if is_sunray { 'S' } else { 'A' })
                .collect();

            PlanetRowVm {
                id: *id,
                status: format!("{:?}", info.status),
                rocket: info.rocket.to_string(),
                energy,
                incoming,
                highlight_neighbor: is_neighbor_of_selection(app, *id),
            }
        })
        .collect()
}

fn status_text(status: Status) -> &'static str {
    match status {
        Status::Running => "Running",
        Status::Paused => "Paused",
        Status::Dead => "Dead",
    }
}

fn energy_bar(charged: usize, total: usize) -> String {
    "■".repeat(charged) + &"□".repeat(total.saturating_sub(charged))
}

fn is_neighbor_of_selection(app: &App, row_planet_id: u32) -> bool {
    match (app.planet_selector.selected(), app.explorer_selector.selected()) {
        (Some(selected_planet), None) => app.galaxy_topology[row_planet_id as usize][selected_planet],
        (None, Some(selected_explorer)) => {
            let explorer_planet = app
                .explorers_info
                .get(&(selected_explorer as u32))
                .map(|e| e.current_planet_id);
            match explorer_planet {
                Some(pid) => app.galaxy_topology[row_planet_id as usize][pid as usize],
                None => false,
            }
        }
        _ => false,
    }
}

