use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

/// Renders the starting screen with title and instructions
///
/// Shows the game title "One Million Crabs Galaxy" and prompts
/// to press ENTER to start or Q to quit
pub fn render_start_screen(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // Create a centered layout
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical_layout[1]);

    let center_area = horizontal_layout[1];

    // Title and control instructions
    let title_text = vec![
        Line::from(Span::styled(
            "   ONE MILLION CRABS GALAXY       ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled("    Press ", Style::default().fg(Color::Gray)),
            Span::styled(
                "ENTER",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to ", Style::default().fg(Color::Gray)),
            Span::styled(
                "START",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("    Press ", Style::default().fg(Color::Gray)),
            Span::styled(
                "Q",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to ", Style::default().fg(Color::Gray)),
            Span::styled(
                "QUIT",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let title = Paragraph::new(title_text).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .style(Style::default()),
    );

    frame.render_widget(title, center_area);
}
