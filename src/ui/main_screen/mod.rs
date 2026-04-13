mod explorers;
mod global;
mod instructions;
mod log;
mod planets;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

/// Main entry point for the game screen UI.
/// Organizes the screen into a top bar for globals and a main grid for tables and info.
pub(crate) fn render_game_ui(app: &mut App, frame: &mut Frame) {
    // --- Layout Definition ---
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Global variables
            Constraint::Fill(1),   // Main content
        ])
        .split(frame.area());

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Left: Tables
            Constraint::Percentage(60), // Right: Details & Log
        ])
        .split(outer_layout[1]);

    let left_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Fill(1),
        ])
        .split(main_layout[0]);

    let right_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Info area
            Constraint::Percentage(40), // Instructions
        ])
        .split(main_layout[1]);

    // --- Component Rendering ---
    global::render_globals_info(app, frame, outer_layout[0]);
    explorers::render_explorers(app, frame, left_column[0]);
    planets::render_planets_table(app, frame, left_column[1]);

    // Logic to switch between different info panels based on selection
    match (app.explorer_selector.selected(), app.planet_selector.selected()) {
        (Some(_), _) => render_extra_info_explorer(app, frame, right_column[0]),
        (None, Some(_)) => render_extra_info_planet(app, frame, right_column[0]),
        (None, None) => render_extra_info_none(app, frame, right_column[0]),
    }

    instructions::render_instructions(app, frame, right_column[1]);

    if app.show_log_overlay {
        log::render_log_overlay(app, frame, main_layout[1]);
    }
}

/// Renders detailed explorer info and an input field for planet ID at the bottom.
fn render_extra_info_explorer(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Extra Info - Explorer ")
        .border_style(Style::default().fg(Color::DarkGray));
    
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Internal layout to push the Destination ID to the bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),   // Top: Static info
            Constraint::Length(1), // Bottom: Dynamic input
        ])
        .split(inner_area);

    // 1. Details Section
    let details = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ID Explorer: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", app.get_id_selected_explorer()), Style::default().bold()),
        ]),
        Line::from(vec![
            Span::styled("  Current Planet: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_planet_selected_explorer(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("  Bag: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_bag_selected_explorer(), Style::default()),
        ]),
    ];
    frame.render_widget(Paragraph::new(details), chunks[0]);

    // 2. Input Section (Destination)
    let typed_id = app.planet_typed.map(|id| id.to_string()).unwrap_or_else(|| "---".to_string());
    let input_line = Line::from(vec![
        Span::styled("  Destination Planet ID: ", Style::default().fg(Color::Cyan)),
        Span::styled(typed_id, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]);
    frame.render_widget(Paragraph::new(input_line), chunks[1]);
}

/// Renders planet details when a planet is selected in the table.
fn render_extra_info_planet(app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Name: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_name_selected_planet(), Style::default().bold()),
        ]),
        Line::from(vec![
            Span::styled("  ID: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_id_selected_planet(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("  Cells: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_cells_info_selected_planet(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("  Rocket: ", Style::default().fg(Color::Gray)),
            Span::styled(format!("{}", app.get_rocket_of_selected_planet()), Style::default()),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Extra Info - Planet ")
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(paragraph, area);
}

/// Helper for rendering the empty state of the info panel.
fn render_extra_info_none(_app: &App, frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new(Line::from("  No Entity Selected")).block(
        Block::bordered()
            .title(" Extra Info ")
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(paragraph, area);
}