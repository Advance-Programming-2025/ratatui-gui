use omc_galaxy::Status;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
};

use crate::app::App;

pub fn render_explorers(app: &App, frame: &mut Frame, area: Rect) {
    let header = Row::new(vec!["ID", "Status", "Bag", "Planet"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .explorers_info
        .iter()
        .map(|(id, info)| {
            let status = match info.status {
                Status::Running => "Running",
                Status::Paused => "Paused",
                Status::Dead => "Dead",
            };
            let mut bag = String::new();
            for i in info.bag.iter() {
                if i.is_aipartner() {
                    bag += "[AP]";
                } else if i.is_carbon() {
                    bag += "[C]";
                } else if i.is_diamond() {
                    bag += "[D]";
                } else if i.is_dolphin() {
                    bag += "[DP]";
                } else if i.is_hydrogen() {
                    bag += "[H]";
                } else if i.is_life() {
                    bag += "[L]";
                } else if i.is_oxygen() {
                    bag += "[O]";
                } else if i.is_robot() {
                    bag += "[R]";
                } else if i.is_silicon() {
                    bag += "[S]";
                } else if i.is_water() {
                    bag += "[W]";
                } else {
                    bag += "[?]";
                }
            }

            let planet_id = info.current_planet_id.to_string();

            Row::new(vec![
                Cell::from(id.to_string()),
                Cell::from(status.to_string()),
                Cell::from(bag),
                Cell::from(planet_id),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Fill(3),
            Constraint::Fill(2),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .title(" Explorers ")
            .border_style(Style::default().fg(Color::LightRed)),
    );

    frame.render_widget(table, area);
}
