mod explorers;
mod global;
mod instructions;
mod log;
mod planets;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;
use crate::ui::theme::{BlockThemeExt, SpanThemeExt, Theme};

/// Main entry point for the game screen UI.
/// Organizes the screen into a top bar for globals and a main grid for tables and info.
pub(crate) fn render_game_ui(app: &mut App, frame: &mut Frame, theme: &Theme) {
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
        .constraints([Constraint::Length(6), Constraint::Fill(1)])
        .split(main_layout[0]);

    let right_column = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60), // Info area
            Constraint::Percentage(40), // Instructions
        ])
        .split(main_layout[1]);

    global::render_globals_info(app, frame, outer_layout[0], theme);
    explorers::render_explorers(app, frame, left_column[0], theme);
    planets::render_planets_table(app, frame, left_column[1], theme);

    // Logic to switch between different info panels based on selection
    match (app.explorer_selector.selected(), app.planet_selector.selected()) {
        (Some(_), _) => render_extra_info_explorer(app, frame, right_column[0], theme),
        (None, Some(_)) => render_extra_info_planet(app, frame, right_column[0], theme),
        (None, None) => render_extra_info_none(frame, right_column[0], theme),
    }

    instructions::render_instructions(app, frame, right_column[1], theme);

    if app.show_log_overlay {
        log::render_log_overlay(app, frame, main_layout[1], theme);
    }
}

/// Renders detailed explorer info and an input field for planet ID at the bottom.
fn render_extra_info_explorer(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let block = Block::bordered()
        .title(" Extra Info - Explorer ")
        .panel(theme);
    
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
            Span::raw("  ID Explorer: ").muted(theme),
            Span::styled(format!("{}", app.get_id_selected_explorer()), theme.value().bold()),
        ]),
        Line::from(vec![
            Span::raw("  Current Planet: ").muted(theme),
            Span::styled(app.get_planet_selected_explorer(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Bag: ").muted(theme),
            Span::styled(app.get_bag_selected_explorer(), theme.value()),
        ]),
    ];
    frame.render_widget(Paragraph::new(details), chunks[0]);

    // 2. Input Section (Destination)
    let typed_id = app.planet_typed.map(|id| id.to_string()).unwrap_or_else(|| "---".to_string());
    let input_line = Line::from(vec![
        Span::raw("  Destination Planet ID: ").accent(theme),
        Span::styled(typed_id, theme.warning().add_modifier(Modifier::BOLD)),
    ]);
    frame.render_widget(Paragraph::new(input_line), chunks[1]);
}

/// Renders planet details when a planet is selected in the table.
fn render_extra_info_planet(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Name: ").muted(theme),
            Span::styled(app.get_name_selected_planet(), theme.value().bold()),
        ]),
        Line::from(vec![
            Span::raw("  ID: ").muted(theme),
            Span::styled(app.get_id_selected_planet(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Cells: ").muted(theme),
            Span::styled(app.get_cells_info_selected_planet(), theme.value()),
        ]),
        Line::from(vec![
            Span::raw("  Rocket: ").muted(theme),
            Span::styled(format!("{}", app.get_rocket_of_selected_planet()), theme.value()),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Extra Info - Planet ")
            .panel(theme),
    );
    frame.render_widget(paragraph, area);
}

/// Helper for rendering the empty state of the info panel.
fn render_extra_info_none(frame: &mut Frame, area: Rect, theme: &Theme) {
    let paragraph = Paragraph::new(Line::from("  No Entity Selected")).block(
        Block::bordered()
            .title(" Extra Info ")
            .panel(theme),
    );
    frame.render_widget(paragraph, area);
}
