use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

/// Render log overlay panel.
/// Overlays right-side area with recent log lines.
pub(crate) fn render_log_overlay(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let logs_lock = app.log_entries.logs.lock().unwrap();

    let mut lines: Vec<Line> = logs_lock
        .iter()
        .map(|(level, msg)| {
            let level_style = theme.default();

            Line::from(vec![
                Span::styled(format!("{:<5} ", level), level_style),
                Span::styled(msg.clone(), theme.value()),
            ])
        })
        .collect();

    // Aggiungi istruzioni in fondo
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("Press ").default(theme),
        Span::styled("L", theme.success().bold()),
        Span::raw(" to close").default(theme),
    ]));

    let log_overlay = Paragraph::new(lines)
        .block(
            Block::bordered()
                .title(" Game Logs")
                .border_style(theme.warning().bold()),
        )
        .style(theme.value())
        .wrap(Wrap { trim: true });

    frame.render_widget(log_overlay, area);
}
