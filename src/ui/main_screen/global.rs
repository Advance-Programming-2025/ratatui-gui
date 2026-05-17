use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};

/// Render global status bar (top panel).
/// Shows game state, simulation clock, and sunray rate.
pub fn render_globals_info(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let outer = Block::default()
        .borders(Borders::ALL)
        .border_style(theme.border());
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let [state_area, time_area, sunray_area] = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .areas(inner);

    let state_line = Line::from(vec![
        Span::raw("Game ").muted(theme),
        Span::styled(
            format!("{:?}", app.gamestate),
            theme.game_state_style(app.gamestate.clone()),
        ),
    ]);
    let state = Paragraph::new(state_line).alignment(Alignment::Center);
    frame.render_widget(state, state_area);

    let time_line = Line::from(vec![
        Span::raw("Time ").muted(theme),
        Span::styled(app.sim_time_hms(), theme.value().bold()),
    ]);
    let time = Paragraph::new(time_line).alignment(Alignment::Center);
    frame.render_widget(time, time_area);

    let sunray_line = Line::from(vec![
        Span::raw("Sunray ").muted(theme),
        Span::styled(format!("{}%", app.sunray_rate), theme.value().bold()),
    ]);
    let sunray = Paragraph::new(sunray_line).alignment(Alignment::Center);
    frame.render_widget(sunray, sunray_area);
}
