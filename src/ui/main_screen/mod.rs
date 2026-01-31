mod log;
mod explorers;
mod planets;
mod global;
mod instructions;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub(crate) fn render_game_ui(app: &mut App, frame: &mut Frame) {
    // Layout principale: 2 righe (Header | Main)
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Global variables
            Constraint::Fill(1),    // Main content
        ])
        .split(frame.area());

    // Layout principale: 2 colonne (Left 40% | Right 60%)
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),  // Left column (Explorers + Planets)
            Constraint::Percentage(60),  // Right column (Extra + Instructions)
        ])
        .split(outer_layout[1]);

    // Left column: Explorers sopra, Planets sotto
    let left_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),   // Explorers (max 2, compatti)
            Constraint::Fill(1),     // Planets (scrollabile)
        ])
        .split(main_layout[0]);

    // Right column: Extra Info sopra, Instructions sotto
    let right_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),  // Extra info
            Constraint::Percentage(40),  // Instructions
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
    render_extra_info(app, frame, right_column[0]);

    // 5. Instructions (bottom right)
    instructions::render_instructions(app, frame, right_column[1]);

    // 6. Log Overlay (se attivo, copre solo la colonna destra)
    if app.show_log_overlay {
        log::render_log_overlay(app, frame, main_layout[1]);
    }
}
fn render_extra_info(app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Extra Stats & Info",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Selected Planet: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.planet_id_selector.map_or("None".to_string(), |id| id.to_string())),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Log Overlay: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app.show_log_overlay { "ON" } else { "OFF" },
                Style::default().fg(if app.show_log_overlay { Color::Green } else { Color::Red }).add_modifier(Modifier::BOLD),
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
