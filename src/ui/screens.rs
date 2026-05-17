use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};

/// Render start screen (pre-game).
/// Shows intro text and generation mode selection.
pub fn render_start_screen(app: &App, frame: &mut Frame, theme: &Theme) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Fill(1),   // Content (Intro + Menu)
            Constraint::Length(3), // Footer / Controls
        ])
        .split(area);

    render_title(frame, chunks[0], theme);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(40), // Intro Text
            Constraint::Percentage(60), // Menu Selection
        ])
        .split(chunks[1]);

    render_intro_text(frame, content_chunks[0], theme);
    render_generation_menu(app, frame, content_chunks[1], theme);

    render_footer(frame, chunks[2], theme);
}

/// Render top title bar for start screen.
fn render_title(frame: &mut Frame, area: Rect, theme: &Theme) {
    let title = Paragraph::new("ONE MILLION CRABS GALAXY")
        .alignment(Alignment::Center)
        .style(theme.accent().add_modifier(Modifier::BOLD))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(theme.border()),
        );
    frame.render_widget(title, area);
}

/// Render mission briefing text panel.
fn render_intro_text(frame: &mut Frame, area: Rect, theme: &Theme) {
    let intro = "Welcome Commander. You are tasked with overseeing the crab-driven expansion across the sector. \
                  Manage explorers, collect rare resources, and maintain planetary equilibrium. \
                  Your journey begins with the formation of the galaxy topology.";

    let p = Paragraph::new(intro)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Mission Briefing ")
                .title_alignment(Alignment::Center)
                .border_style(theme.border()),
        );
    frame.render_widget(p, area);
}

/// Render mode selection menu.
/// Reads selection state from `App` and highlights active choice.
fn render_generation_menu(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let is_random = app.ui.start.selected_mode == 0;
    let is_custom = app.ui.start.selected_mode == 1;

    let random_style = if is_random {
        theme.header()
    } else {
        theme.muted()
    };
    let custom_style = if is_custom {
        theme.header()
    } else {
        theme.muted()
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
                format!(
                    "CUSTOM GENERATION (Count: {})",
                    app.ui.start.custom_planet_count
                ),
                custom_style,
            ),
        ]),
    ];

    let menu = Paragraph::new(menu_items)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .title(" Galaxy Generation Mode ")
                .border_style(theme.border()),
        );

    frame.render_widget(menu, area);
}

/// Render footer with key hints.
fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let info = Line::from(vec![
        Span::styled(" UP/DOWN ", theme.key()),
        Span::raw("Navigate | ").value(theme),
        Span::styled(" ENTER ", theme.success().add_modifier(Modifier::BOLD)),
        Span::raw("Confirm & Start | ").value(theme),
        Span::styled(" Q ", theme.danger()),
        Span::raw("Quit").value(theme),
    ]);
    frame.render_widget(Paragraph::new(info).alignment(Alignment::Center), area);
}
