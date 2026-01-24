use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Render the starting screen with title and instructions
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

    // Title and ASCII art
    let title_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "╔═══════════════════════════════════════╗",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "║                                       ║",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "║    ONE MILLION CRABS GALAXY 🦀        ║",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "║                                       ║",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "╚═══════════════════════════════════════╝",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "         Welcome, Commander!",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "    Your mission: Explore the galaxy,",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "    manage planets, and guide explorers",
            Style::default().fg(Color::White),
        )),
        Line::from(Span::styled(
            "    through the cosmic wilderness.",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("    Press ", Style::default().fg(Color::Gray)),
            Span::styled("S", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" to ", Style::default().fg(Color::Gray)),
            Span::styled("START", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("    Press ", Style::default().fg(Color::Gray)),
            Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(" to ", Style::default().fg(Color::Gray)),
            Span::styled("QUIT", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let title = Paragraph::new(title_text)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .style(Style::default().bg(Color::Black)),
        );

    frame.render_widget(title, center_area);
}

/// Render a pause overlay on top of the existing UI
pub fn render_pause_overlay(app: &App, frame: &mut Frame) {
    let area = frame.area();

    // Create a centered overlay
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ])
        .split(area);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(vertical_layout[1]);

    let overlay_area = horizontal_layout[1];

    let pause_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "      ╔══════════════════════╗",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "      ║                      ║",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "      ║    ⏸  PAUSED  ⏸     ║",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "      ║                      ║",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "      ╚══════════════════════╝",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("         Press ", Style::default().fg(Color::Gray)),
            Span::styled("P", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" to Resume", Style::default().fg(Color::Gray)),
        ]),
        Line::from(""),
    ];

    let pause_overlay = Paragraph::new(pause_text)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .style(Style::default().bg(Color::Black)),
        );

    frame.render_widget(pause_overlay, overlay_area);
}