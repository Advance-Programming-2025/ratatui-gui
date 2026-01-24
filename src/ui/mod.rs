mod debug;
mod explorers;
mod planets;
mod screens;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::app::App;
use crate::states::GameState;

pub fn render_ui(app: &App, frame: &mut Frame) {
    match app.get_game_state() {
        GameState::WaitingStart => {
            // Show start screen
            screens::render_start_screen(app, frame);
        }
        GameState::Running => {
            // Show normal game UI
            render_game_ui(app, frame);
        }
        GameState::Paused => {
            // Show game UI with pause overlay
            render_game_ui(app, frame);
            screens::render_pause_overlay(app, frame);
        }
    }
}

fn render_game_ui(app: &App, frame: &mut Frame) {
    // Main Layout: 3 columns. 1.Planets 2.1 Explorers 2.2 Instructions 3.Debug Messages
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(frame.area());

    // Planets Area
    let left_layout = main_layout[0];

    // Up Explorers and under Instructions
    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    // Debug Messages Area
    let debug_area = main_layout[2];

    // 1. RENDERING PLANETS TABLE
    planets::render_planets_table(app, frame, left_layout);

    // 2. RENDERING EXPLORERS (LIST)
    explorers::render_explorers(app, frame, right_layout[0]);

    // 3. RENDERING INSTRUCTIONS
    let inst = Paragraph::new("Q: Quit | S: Start | P: Pause").block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(inst, right_layout[1]);

    // 4. RENDERING DEBUG MESSAGES
    debug::render_debug_messages(app, frame, debug_area);
}