use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, game_state::GameState};

//import matrix color
use super::*;

pub fn render_globals_info(app: &App, frame: &mut Frame, area: Rect) {
    let title_text = vec![Line::from(vec![
        Span::styled("Game: ", Style::default().fg(MATRIX_GREEN)),
        Span::styled(
            format!("{:?}", app.gamestate),
            match app.gamestate {
                GameState::Running => Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
                GameState::Paused => Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
                GameState::Ended => Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
                GameState::WaitingStart => Style::default()
                    .fg(MATRIX_GREEN)
                    .add_modifier(Modifier::BOLD),
            },
        ),
        Span::styled(" | ", Style::default().fg(MATRIX_GREEN)),
        Span::styled("Simulation Time: ", Style::default().fg(MATRIX_GREEN)),
        Span::styled(
            format!("???",),
            Style::default()
                .fg(MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(MATRIX_GREEN)),
        Span::styled("Total Planets: ", Style::default().fg(MATRIX_GREEN)),
        Span::styled(
            format!("{}", app.planets_info.len()),
            Style::default()
                .fg(MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(MATRIX_GREEN)),
        Span::styled("Total Explorers: ", Style::default().fg(MATRIX_GREEN)),
        Span::styled(
            format!("{}", app.explorers_info.len()),
            Style::default()
                .fg(MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(MATRIX_GREEN)),
        Span::styled("Sunray%: ", Style::default().fg(MATRIX_GREEN)),
        Span::styled(
            format!("{}%", app.sunray_rate),
            Style::default()
                .fg(MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(MATRIX_GREEN)),
        Span::styled("Sunray%: ", Style::default().fg(MATRIX_GREEN)),
        Span::styled(
            format!("{}%", app.sunray_rate),
            Style::default()
                .fg(MATRIX_GREEN)
                .add_modifier(Modifier::BOLD),
        ),
    ])];

    let title = Paragraph::new(title_text).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MATRIX_GREEN))
            .style(Style::default()),
    );
    frame.render_widget(title, area);
}
