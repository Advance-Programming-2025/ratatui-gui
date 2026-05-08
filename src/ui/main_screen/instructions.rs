use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;

//import matrix color
use super::*;

pub(crate) fn render_instructions(_app: &App, frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Q ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("- Quit", Style::default().fg(MATRIX_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(
                "  P ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("- Pause/Resume", Style::default().fg(MATRIX_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↑ ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Select up Explorer/Planet",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↓ ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Select down Explorer/Planet",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  L ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("- Toggle Log Overlay", Style::default().fg(MATRIX_GREEN)),
        ]),
        Line::from(vec![
            Span::styled(
                "  O ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Increase Sunray %(pause first)",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  I ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Decrease Sunray %(pause first)",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  S ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Send Sunray(select planet first)",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  A ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Send Asteroid(select planet first)",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  1..9 ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Type Destination Planet ID(select explorer first)",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ENTER ",
                Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Confirm Destination Planet ID",
                Style::default().fg(MATRIX_GREEN),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(Style::default().fg(MATRIX_GREEN)),
    );
    frame.render_widget(paragraph, area);
}
