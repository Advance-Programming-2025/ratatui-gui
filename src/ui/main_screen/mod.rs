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
    let [global_area, main_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(frame.area());

    let [planets_area, explorers_area, other_area] = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(40),
    ])
    .areas(main_area);

    let [planets_info_area, planets_list_area] =
        Layout::vertical([Constraint::Percentage(45), Constraint::Fill(1)]).areas(planets_area);

    let [explorers_info_area, explorers_list_area] =
        Layout::vertical([Constraint::Percentage(45), Constraint::Fill(1)]).areas(explorers_area);

    global::render_globals_info(app, frame, global_area, theme);
    explorers::render_explorers(app, frame, explorers_list_area, theme);
    planets::render_planets_table(app, frame, planets_list_area, theme);

    match (
        app.general_selector.get_last_explorer_selected(),
        app.general_selector.get_last_planet_selected(),
    ) {
        (Some(_), Some(_))=>{
            render_extra_info_explorer(app, frame, explorers_info_area, theme);
            render_extra_info_planet(app, frame, planets_info_area, theme);
        }
        (Some(_), _) => {
            render_extra_info_explorer(app, frame, explorers_info_area, theme);
            render_extra_info_none(frame, planets_info_area, theme);
        }
        (None, Some(_)) => {
            render_extra_info_none(frame, explorers_info_area, theme);
            render_extra_info_planet(app, frame, planets_info_area, theme);
        }
        (None, None) => {
            render_extra_info_none(frame, planets_info_area, theme);
            render_extra_info_none(frame, explorers_info_area, theme);
        }
    }

    instructions::render_instructions(app, frame, other_area, theme);

    if app.show_log_overlay {
        log::render_log_overlay(app, frame, other_area, theme);
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
            Span::styled(
                format!("{}", app.get_id_selected_explorer()),
                theme.value().bold(),
            ),
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
    let typed_id = app
        .planet_typed
        .map(|id| id.to_string())
        .unwrap_or_else(|| "---".to_string());
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
            Span::styled(
                format!("{}", app.get_rocket_of_selected_planet()),
                theme.value(),
            ),
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
    let paragraph = Paragraph::new(Line::from("  No Entity Selected"))
        .style(theme.value())
        .block(Block::bordered().title(" Extra Info ").panel(theme));
    frame.render_widget(paragraph, area);
}
