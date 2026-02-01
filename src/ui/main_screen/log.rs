use crate::app::App;
use log::Level;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

/// Render overlay dei log che copre solo la colonna destra
pub fn render_log_overlay(app: &App, frame: &mut Frame, area: Rect) {
    let logs_lock = app.log_entries.logs.lock().unwrap();

    let mut lines: Vec<Line> = logs_lock
        .iter()
        .map(|(level, msg)| {
            let color = match *level {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Cyan,
                Level::Trace => Color::DarkGray,
            };

            Line::from(vec![
                Span::styled(
                    format!("{:<5} ", level),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(msg.clone(), Style::default().fg(Color::White)),
            ])
        })
        .collect();

    // Aggiungi istruzioni in fondo
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Press ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "L",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" to close", Style::default().fg(Color::DarkGray)),
    ]));

    let log_overlay = Paragraph::new(lines)
        .block(
            Block::bordered().title(" Game Logs").border_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .style(Style::default()) // Background nero per contrasto
        .wrap(Wrap { trim: true });

    frame.render_widget(log_overlay, area);
}
