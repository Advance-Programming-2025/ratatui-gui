use omc_galaxy::Status;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, Table},
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
            let bag = "[ ]".repeat(5);
            let planet_id = info.current_planet_id.to_string();

            Row::new(vec![
                Cell::from(id.to_string()),
                Cell::from(status.to_string()),
                Cell::from(bag),
                Cell::from(planet_id),
            ])
        })
        .collect();
    // let items: Vec<ListItem> = app
    //     .explorers
    //     .iter()
    //     .map(|e| {
    //         let mut inv = String::new();
    //         for i in 0..5 {
    //             inv.push_str(if i < e.inventory.len() { "[X]" } else { "[ ]" });
    //         }
    //         ListItem::new(format!(
    //             "Explorer {} @ {} | Inventory: {}",
    //             e.id, e.pos, inv
    //         ))
    //     })
    //     .collect();

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
    .block(Block::bordered().title(" Explorers ").border_style(Style::default().fg(Color::LightRed)));

    frame.render_widget(table, area);
}
