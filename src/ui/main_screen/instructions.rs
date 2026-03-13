use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub(crate) fn render_instructions(_app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Q ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("- Quit", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  P ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("- Pause/Resume", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↑ ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Select up Explorer/Planet",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↓ ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Select down Explorer/Planet",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  L ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("- Toggle Log Overlay", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled(
                "  O ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Increase Sunray %(pause first)",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  I ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Decrease Sunray %(pause first)",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  S ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Send Sunray(select a planet first)",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  A ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Send Asteroid(select a planet first)",
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(paragraph, area);
}
