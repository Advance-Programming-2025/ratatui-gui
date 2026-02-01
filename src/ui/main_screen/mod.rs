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

pub(crate) fn render_game_ui(app: &mut App, frame: &mut Frame) {
    // Layout principale: 2 righe (Header | Main)
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Global variables
            Constraint::Fill(1),   // Main content
        ])
        .split(frame.area());

    // Layout principale: 2 colonne (Left 40% | Right 60%)
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // Left column (Explorers + Planets)
            Constraint::Percentage(60), // Right column (Extra + Instructions)
        ])
        .split(outer_layout[1]);

    // Left column: Explorers sopra, Planets sotto
    let left_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Explorers (max 2, compatti)
            Constraint::Fill(1),   // Planets (scrollabile)
        ])
        .split(main_layout[0]);

    // Right column: Extra Info sopra, Instructions sotto
    let right_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Extra info
            Constraint::Percentage(40), // Instructions
        ])
        .split(main_layout[1]);

    /////////// RENDERING SECTIONS //////////////////

    // 1. Global variables (top)
    global::render_globals_info(app, frame, outer_layout[0]);

    // 2. Explorers (top left)
    explorers::render_explorers(app, frame, left_column[0]);

    // 3. Planets (bottom left)
    planets::render_planets_table(app, frame, left_column[1]);

    // 4. Extra Info (top right)
    render_extra_info_planet(app, frame, right_column[0]);

    // 5. Instructions (bottom right)
    instructions::render_instructions(app, frame, right_column[1]);

    // 6. Log Overlay (se attivo, copre solo la colonna destra)
    if app.show_log_overlay {
        log::render_log_overlay(app, frame, main_layout[1]);
    }
}

fn render_extra_info_planet(app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Select Entity",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Name Planet: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.get_name_selected_planet()),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Planet ID: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_id_selected_planet(), Style::default()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Charged Cells: ", Style::default().fg(Color::Gray)),
            Span::styled(app.get_cells_info_selected_planet(), Style::default()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Rocket: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.get_rocket_of_selected_planet()),
                Style::default(),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Extra Info ")
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(paragraph, area);
}
