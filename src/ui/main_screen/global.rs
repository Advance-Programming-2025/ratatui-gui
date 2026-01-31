use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, game_state::GameState};

pub fn render_globals_info(app: &App, frame: &mut Frame, area: Rect) {
    let title_text = vec![Line::from(vec![
        Span::styled("Game: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{:?}",app.gamestate),
            match app.gamestate{
                GameState::Running=>{
                    Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD)
                },
                GameState::Paused=>{
                    Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                }
                GameState::Ended=>{
                    Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
                },
                GameState::WaitingStart=>{
                    Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD)
                }
                
            }
            
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Simulation Time: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("???",),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Total Planets: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", app.planets_info.len()),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Total Explorers: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", app.explorers_info.len()),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Sunray%: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}%", app.probability_sunray),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])];
    
    let title = Paragraph::new(title_text).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .style(Style::default()),
    );
    frame.render_widget(title, area);
}
