use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};

/// Render global status bar (top panel).
/// Shows game state and a few aggregate counters.
pub fn render_globals_info(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let title_text = vec![Line::from(vec![
        Span::raw("Game: ").muted(theme),
        Span::styled(
            format!("{:?}", app.gamestate),
            theme.game_state_style(app.gamestate.clone()),
        ),
        Span::raw(" | ").muted(theme),
        Span::raw("Simulation Time: ").muted(theme),
        Span::styled("???", theme.value().bold()),
        Span::raw(" | ").muted(theme),
        Span::raw("Total Planets: ").muted(theme),
        Span::styled(format!("{}", app.planets_info.len()), theme.value().bold()),
        Span::raw(" | ").muted(theme),
        Span::raw("Total Explorers: ").muted(theme),
        Span::styled(
            format!("{}", app.explorers_info.len()),
            theme.value().bold(),
        ),
        Span::raw(" | ").muted(theme),
        Span::raw("Sunray%: ").muted(theme),
        Span::styled(format!("{}%", app.sunray_rate), theme.value().bold()),
    ])];

    let title = Paragraph::new(title_text).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border())
            .style(theme.value()),
    );
    frame.render_widget(title, area);
}
