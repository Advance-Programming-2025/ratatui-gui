use omc_galaxy::Status;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
};

use crate::app::App;
use crate::app::bag_to_string;

use super::*;

pub(crate) fn render_explorers(app: &mut App, frame: &mut Frame, area: Rect) {
    let header = Row::new(vec!["ID", "Status", "Bag", "Planet"]).style(
        Style::default()
            .fg(MATRIX_GREEN)
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
            let bag = bag_to_string(&info.bag);

            let planet_id = info.current_planet_id.to_string();

            Row::new(vec![
                Cell::from(id.to_string()),
                Cell::from(status.to_string()),
                Cell::from(bag),
                Cell::from(planet_id),
            ])
            .fg(MATRIX_GREEN)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Fill(3),
            Constraint::Length(7),
        ],
    )
    .header(header)
    .block(
        Block::bordered()
            .title(" Explorers ")
            .border_style(Style::default().fg(MATRIX_GREEN)),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray).fg(MATRIX_GREEN));

    frame.render_stateful_widget(table, area, &mut app.explorer_selector);
}
