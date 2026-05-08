use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
};

use crate::app::{App, get_status_text_color_tuple};

pub fn render_planets_table(app: &mut App, frame: &mut Frame, area: Rect) {
    let header = Row::new(vec!["ID", "Status", "Rocket", "Energy", "Incoming"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    // TODO: discriminate between the number of energy cells
    let rows: Vec<Row> = app
        .planets_info
        .iter()
        .map(|(id, info)| {
            let energy_str = "■".repeat(info.charged_cells_count)
                + &"□".repeat(info.energy_cells.len() - info.charged_cells_count);

            // Row style: write in Red if it is a neighbours of the selected planet
            let row_style = match (
                app.planet_selector.selected(),
                app.explorer_selector.selected(),
            ) {
                (Some(planet), None) => {
                    if app.galaxy_topology[*id as usize][planet] {
                        Style::default().fg(Color::Red).bold()
                    } else {
                        Style::default()
                    }
                }
                (None, Some(explorer)) => {
                    let planet = app
                        .explorers_info
                        .get(&(explorer as u32))
                        .unwrap()
                        .current_planet_id;
                    if app.galaxy_topology[*id as usize][planet as usize] {
                        Style::default().fg(Color::Red).bold()
                    } else {
                        Style::default()
                    }
                }
                (_, _) => Style::default(),
            };

            let status = get_status_text_color_tuple(info.status);

            let incoming: String = app
                .find_incoming_sunray_asteroid_for_planet(*id)
                .iter()
                .map(|&is_sunray| if is_sunray { 'S' } else { 'A' })
                .collect();

            Row::new(vec![
                Cell::from(id.to_string()),
                Cell::from(status.to_string()),
                Cell::from(info.rocket.to_string()),
                Cell::from(energy_str),
                Cell::from(incoming),
            ])
            .style(row_style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Min(7),
            Constraint::Min(7),
            Constraint::Min(7),
            Constraint::Min(7),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .title(" Planets ")
            .border_style(Style::default().fg(Color::Green)),
    )
    // AGGIUNTA: Definiamo lo stile della riga selezionata centralmente
    .row_highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White));

    // CAMBIO: Usa render_stateful_widget invece di render_widget
    frame.render_stateful_widget(table, area, &mut app.planet_selector);
}
