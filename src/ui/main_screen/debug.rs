use crate::app::App;
use log::Level;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
}; // Assicurati di importare Level

pub fn render_debug_messages(app: &App, frame: &mut Frame, area: Rect) {
    let logs_lock = app.log_entries.logs.lock().unwrap();

    // Trasformiamo la collezione di tuple in un vettore di Linee per Ratatui
    let lines: Vec<Line> = logs_lock
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
                // Spacchettiamo la tupla e coloriamo il livello
                Span::styled(
                    format!("{:<5} ", level),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ),
                // Il messaggio lo lasciamo bianco o grigio
                Span::styled(msg.clone(), Style::default().fg(Color::White)),
            ])
        })
        .collect(); // <--- Qui ora collect produce un Vec<Line>, che Paragraph accetta

    let debug_paragraph = Paragraph::new(lines) // Paragraph accetta Vec<Line>
        .block(
            Block::bordered()
                .title(" Debug Logs ")
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(debug_paragraph, area);
}
