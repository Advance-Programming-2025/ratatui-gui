use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

/// Renders the multi-step start screen.
/// Includes game introduction and galaxy generation settings.
pub fn render_start_screen(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Fill(1),   // Content (Intro + Menu)
            Constraint::Length(3), // Footer / Controls
        ])
        .split(area);

    // --- 1. Header ---
    render_title(frame, chunks[0]);

    // --- 2. Central Content ---
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(40), // Intro Text
            Constraint::Percentage(60), // Menu Selection
        ])
        .split(chunks[1]);

    render_intro_text(frame, content_chunks[0]);
    render_generation_menu(app, frame, content_chunks[1]);

    // --- 3. Footer ---
    render_footer(frame, chunks[2]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("ONE MILLION CRABS GALAXY")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(title, area);
}

fn render_intro_text(frame: &mut Frame, area: Rect) {
    let intro = "Welcome Commander. You are tasked with overseeing the crab-driven expansion across the sector. \
                  Manage explorers, collect rare resources, and maintain planetary equilibrium. \
                  Your journey begins with the formation of the galaxy topology.";

    let p = Paragraph::new(intro)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Mission Briefing ")
                .title_alignment(Alignment::Center),
        );
    frame.render_widget(p, area);
}

fn render_generation_menu(app: &App, frame: &mut Frame, area: Rect) {
    // Note: Assuming app.selected_mode: 0 = Random, 1 = Custom
    // Assuming app.custom_planet_count for the custom input display

    let is_random = app.selected_mode == 0;
    let is_custom = app.selected_mode == 1;

    let random_style = if is_random {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let custom_style = if is_custom {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let menu_items = vec![
        Line::from(vec![
            Span::styled(if is_random { " > " } else { "   " }, random_style),
            Span::styled("RANDOM GENERATION (3-20 Planets)", random_style),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(if is_custom { " > " } else { "   " }, custom_style),
            Span::styled(
                format!("CUSTOM GENERATION (Count: {})", app.custom_planet_count),
                custom_style,
            ),
        ]),
    ];

    let menu = Paragraph::new(menu_items)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .title(" Galaxy Generation Mode ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    frame.render_widget(menu, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let info = Line::from(vec![
        Span::styled(" UP/DOWN ", Style::default().fg(Color::Magenta).bold()),
        Span::styled("Navigate | ", Style::default().fg(Color::White)),
        Span::styled(" ENTER ", Style::default().fg(Color::Green).bold()),
        Span::styled("Confirm & Start | ", Style::default().fg(Color::White)),
        Span::styled(" Q ", Style::default().fg(Color::Red).bold()),
        Span::styled("Quit", Style::default().fg(Color::White)),
    ]);
    frame.render_widget(Paragraph::new(info).alignment(Alignment::Center), area);
}
