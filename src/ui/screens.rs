use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};
use tui_big_text::{BigText, PixelSize};

use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};

/// Render start screen (pre-game).
/// Shows intro text and generation mode selection.
pub(crate) fn render_start_screen(app: &App, frame: &mut Frame, theme: &Theme) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1), // Title
            Constraint::Fill(2), // Content (Intro + Menu)
            Constraint::Fill(2),
            Constraint::Length(3), // Footer / Controls
        ])
        .split(area);

    let title_area = chunks[0];
    let mission_area = chunks[1];
    let generation_area = chunks[2];
    let footer_area = chunks[3];

    render_title(frame, title_area, theme);

    render_intro_text(frame, mission_area, theme);
    render_generation_menu(app, frame, generation_area, theme);

    render_footer(frame, footer_area, theme);
}

/// Render top title bar for start screen.
fn render_title(frame: &mut Frame, area: Rect, theme: &Theme) {
    // 1. Disegna il blocco di sfondo con il bordo inferiore
    let block = Block::default().border_style(theme.border());

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    // Scegliamo la dimensione del font.
    // PixelSize::Half è alto 4 righe (ottimo per i titoli).
    let font_height = 4;

    // 2. Centratura VERTICALE
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(font_height), // Spazio per il font grande
            Constraint::Min(0),
        ])
        .split(inner_area);

    let center_vertical_area = vertical_chunks[1];

    // 3. Centratura ORIZZONTALE
    // I widget di tui-big-text si allineano a sinistra di base dentro la loro area.
    // Per centrarlo perfettamente, stimiamo la larghezza del testo.
    // Ogni carattere in PixelSize::Half è largo circa 4/5 colonne.
    let text_width = "ONE MILLION CRABS GALAXY".len() * 4;

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(text_width as u16),
            Constraint::Min(0),
        ])
        .split(center_vertical_area);

    let final_title_area = horizontal_chunks[1];

    // 4. Creazione del BigText
    let big_title = BigText::builder()
        .pixel_size(PixelSize::Sextant) // Altezza: 4 righe. Usa PixelSize::Full per 8 righe.
        .style(theme.default().add_modifier(Modifier::BOLD))
        .lines(vec!["ONE MILLION CRABS GALAXY".into()])
        .build();

    // 5. Renderizza il titolo gigante al centro esatto
    frame.render_widget(big_title, final_title_area);
}

/// Render mission briefing text panel.
fn render_intro_text(frame: &mut Frame, area: Rect, theme: &Theme) {
    let intro = "\nWelcome Commander. You are tasked to simulate the crab-driven mission across the galaxy. \
                  Manage explorers, collect rare resources, and understand how the galaxy works. 
                  ";

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(20),
            Constraint::Fill(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let p = Paragraph::new(intro)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" Mission Briefing ")
                .title_alignment(Alignment::Center)
                .title_style(theme.default().add_modifier(Modifier::BOLD))
                .style(theme.default())
                .border_style(theme.border()),
        );

    frame.render_widget(p, horizontal_chunks[1]);
}

/// Render mode selection menu.
/// Reads selection state from `App` and highlights active choice.
fn render_generation_menu(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let is_random = app.ui.start.selected_mode == 0;
    let is_custom = app.ui.start.selected_mode == 1;

    let random_style = if is_random {
        theme.header()
    } else {
        theme.default()
    };
    let custom_style = if is_custom {
        theme.header()
    } else {
        theme.default()
    };

    let menu_items = vec![
        Line::from(vec![
            Span::styled(if is_random { " > " } else { "   " }, random_style),
            Span::styled(
                "FILE TOPOLOGY GENERATION (Edit galaxy.txt file)",
                random_style,
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(if is_custom { " > " } else { "   " }, custom_style),
            Span::styled(
                format!(
                    "RANDOM GENERATION (Number of planets: {})",
                    app.ui.start.custom_planet_count
                ),
                custom_style,
            ),
        ]),
    ];

    let menu = Paragraph::new(menu_items).alignment(Alignment::Center);

    frame.render_widget(menu, area);
}

/// Render footer with key hints.
fn render_footer(frame: &mut Frame, area: Rect, theme: &Theme) {
    let info = Line::from(vec![
        Span::styled(" UP/DOWN ", theme.key()),
        Span::raw("Navigate | ").default(theme),
        Span::styled(" LEFT/RIGHT ", theme.key()),
        Span::raw("Increase/Decrease | ").default(theme),
        Span::styled(" ENTER ", theme.default().add_modifier(Modifier::BOLD)),
        Span::raw("Confirm & Start | ").default(theme),
        Span::styled(" Q ", theme.danger()),
        Span::raw("Quit").default(theme),
    ]);
    frame.render_widget(Paragraph::new(info).alignment(Alignment::Center), area);
}
