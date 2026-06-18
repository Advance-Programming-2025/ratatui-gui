mod explorers;
mod top_status_bar;
mod instructions;
mod log;
mod planets;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::app::App;
use crate::ui::theme::{BlockThemeExt, Theme};

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

    top_status_bar::render_globals_info(app, frame, global_area, theme);
    explorers::render_explorers(app, frame, explorers_list_area, theme);
    planets::render_planets_table(app, frame, planets_list_area, theme);

    match (
        app.ui.selectors.explorers.last_selected(),
        app.ui.selectors.planets.last_selected(),
    ) {
        (Some(_), Some(_)) => {
            explorers::render_extra_info_explorer(app, frame, explorers_info_area, theme);
            planets::render_extra_info_planet(app, frame, planets_info_area, theme);
        }
        (Some(_), _) => {
            explorers::render_extra_info_explorer(app, frame, explorers_info_area, theme);
            render_extra_info_none(frame, planets_info_area, theme);
        }
        (None, Some(_)) => {
            render_extra_info_none(frame, explorers_info_area, theme);
            planets::render_extra_info_planet(app, frame, planets_info_area, theme);
        }
        (None, None) => {
            render_extra_info_none(frame, planets_info_area, theme);
            render_extra_info_none(frame, explorers_info_area, theme);
        }
    }

    if app.ui.overlays.show_log {
        log::render_log_overlay(app, frame, other_area, theme);
    } else {
        instructions::render_instructions(app, frame, other_area, theme);
    }
}

/// Helper for rendering the empty state of the info panel.
fn render_extra_info_none(frame: &mut Frame, area: Rect, theme: &Theme) {
    let paragraph = Paragraph::new(Line::from("  No Entity Selected"))
        .style(theme.value())
        .block(Block::bordered().title(" Extra Info ").panel(theme));
    frame.render_widget(paragraph, area);
}
