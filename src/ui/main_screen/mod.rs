mod explorers;
mod global;
mod instructions;
mod log;
mod planets;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

//Matrix neon green
pub const MATRIX_GREEN: Color = Color::Rgb(51, 255, 51);

/// Main entry point for the game screen UI.
/// Organizes the screen into a top bar for globals and a main grid for tables and info.
pub(crate) fn render_game_ui(app: &mut App, frame: &mut Frame) {
    // --- Layout Definition ---
    let [global, main_layout] = Layout::vertical([
        Constraint::Length(3), // Global variables
        Constraint::Fill(1),   // Main content
    ])
    .areas(frame.area());

    let [planets, explorers, other] = Layout::horizontal([
        Constraint::Percentage(30), // Left: Tables
        Constraint::Percentage(30), // Right: Details & Log
        Constraint::Percentage(40),
    ])
    .areas(main_layout);

    let [planets_info, planets_list] =
        Layout::vertical([Constraint::Percentage(45), Constraint::Fill(1)]).areas(planets);

    let [explorers_info, explorers_list] = Layout::vertical([
        Constraint::Percentage(45), // Info area
        Constraint::Fill(1),        // Instructions
    ])
    .areas(explorers);

    // --- Component Rendering ---
    global::render_globals_info(app, frame, global);
    explorers::render_explorers(app, frame, explorers_list);
    planets::render_planets_table(app, frame, planets_list);

    // Logic to switch between different info panels based on selection
    match (
        app.explorer_selector.selected(),
        app.planet_selector.selected(),
    ) {
        (Some(_), _) => {
            render_extra_info_explorer(app, frame, explorers_info);
            render_extra_info_none(app, frame, planets_info);
        }
        (None, Some(_)) => {
            render_extra_info_none(app, frame, explorers_info);
            render_extra_info_planet(app, frame, planets_info);
        }
        (None, None) => {
            render_extra_info_none(app, frame, planets_info);
            render_extra_info_none(app, frame, explorers_info);
        }
    }

    instructions::render_instructions(app, frame, other);

    if app.show_log_overlay {
        log::render_log_overlay(app, frame, other);
    }
}

/// Renders detailed explorer info and an input field for planet ID at the bottom.
fn render_extra_info_explorer(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .title(" Explorer ")
        .border_style(Style::default().fg(MATRIX_GREEN));

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
            Span::styled("  ID Explorer: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                format!("{}", app.get_id_selected_explorer()),
                Style::default().fg(MATRIX_GREEN).bold(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Current Planet: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                app.get_planet_selected_explorer(),
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Bag: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                app.get_bag_selected_explorer(),
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
    ];
    frame.render_widget(Paragraph::new(details), chunks[0]);

    // 2. Input Section (Destination)
    let typed_id = app
        .planet_typed
        .map(|id| id.to_string())
        .unwrap_or_else(|| "---".to_string());
    let input_line = Line::from(vec![
        Span::styled(
            "  Destination Planet ID: ",
            Style::default().fg(MATRIX_GREEN),
        ),
        Span::styled(
            typed_id,
            Style::default()
                .fg(MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(Paragraph::new(input_line), chunks[1]);
}

/// Renders planet details when a planet is selected in the table.
fn render_extra_info_planet(app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Name: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                app.get_name_selected_planet(),
                Style::default().fg(MATRIX_GREEN).bold(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ID: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                app.get_id_selected_planet(),
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Cells: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                app.get_cells_info_selected_planet(),
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Rocket: ", Style::default().fg(MATRIX_GREEN)),
            Span::styled(
                format!("{}", app.get_rocket_of_selected_planet()),
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Planet ")
            .border_style(Style::default().fg(MATRIX_GREEN)),
    );
    frame.render_widget(paragraph, area);
}

/// Helper for rendering the empty state of the info panel.
fn render_extra_info_none(_app: &App, frame: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new(Line::from("  No Entity Selected"))
        .fg(MATRIX_GREEN)
        .block(
            Block::bordered()
                .title(" Extra info ")
                .border_style(Style::default().fg(MATRIX_GREEN)),
        );
    frame.render_widget(paragraph, area);
}
