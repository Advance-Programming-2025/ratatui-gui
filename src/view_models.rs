use omc_galaxy::Status;

use crate::app::{App, bag_to_string};

/// Pre-formatted explorer row for table rendering.
/// Keeps UI render code focused on layout and widget composition.
#[derive(Debug, Clone)]
pub struct ExplorerRowVm {
    /// Explorer id.
    pub id: u32,
    /// Human-readable status label.
    pub status: &'static str,
    /// Condensed bag representation.
    pub bag: String,
    /// Current planet id as string.
    pub planet_id: String,
}

/// Pre-formatted planet row for table rendering.
/// Contains both display text and simple render flags.
#[derive(Debug, Clone)]
pub struct PlanetRowVm {
    /// Planet id.
    pub id: u32,
    /// Human-readable status label.
    pub status: String,
    /// Rocket availability text.
    pub rocket: String,
    /// Energy bar string (charged/un-charged cells).
    pub energy: String,
    /// Incoming events string (S for sunray, A for asteroid).
    pub incoming: String,
    /// Neighbor highlight flag for current selection context.
    pub highlight_neighbor: bool,
}

/// Build explorer table rows from current app snapshot.
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

/// Build planet table rows from current app snapshot.
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

/// Status game labels for UI.
fn status_text(status: Status) -> &'static str {
    match status {
        Status::Running => "Running",
        Status::Paused => "Paused",
        Status::Dead => "Dead",
    }
}

/// Energy bar rendering
fn energy_bar(charged: usize, total: usize) -> String {
    "■".repeat(charged) + &"□".repeat(total.saturating_sub(charged))
}

/// Neighbor detection on current explorer or planet selection.
fn is_neighbor_of_selection(app: &App, row_planet_id: u32) -> bool {
    match (
        app.ui.selectors.planets.selected(),
        app.ui.selectors.explorers.selected(),
    ) {
        (Some(selected_planet), None) => {
            app.galaxy_topology[row_planet_id as usize][selected_planet]
        }
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
