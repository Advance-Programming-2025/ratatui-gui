use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};

/// Render help panel with categorized key bindings.
pub(crate) fn render_instructions(_app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let mut text = vec![];

    // --- NOTICE BANNER ---
    if let Some(banner) = _app.ui.overlays.banner.as_ref() {
        text.push(Line::from(vec![
            Span::styled("  Notice: ", theme.default().bold()),
            Span::styled(banner.as_str(), theme.default().bold()),
        ]));
        text.push(Line::from(""));
    }

    // --- 1. SYSTEM & INTERFACE ---
    text.push(Line::from("")); // Separator
    text.push(Line::from(vec![Span::styled(
        " System ",
        theme.accent().bold(),
    )]));
    text.extend([
        Line::from(vec![
            Span::raw(" [Q] ").default(theme),
            Span::raw("- Quit game").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [P] ", theme.default().bold()),
            Span::raw("- Pause / Resume").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [L] ", theme.default().bold()),
            Span::raw("- Toggle Log Overlay").default(theme),
        ]),
    ]);
    text.push(Line::from("")); // Separator
    text.push(Line::from("")); // Separator

    // --- 2. NAVIGATION ---
    text.push(Line::from(vec![Span::styled(
        " Navigation ",
        theme.accent().bold(),
    )]));
    text.extend([
        Line::from(vec![
            Span::styled(" [↑/↓] ", theme.nav()),
            Span::raw("- Select Explorer / Planet / Resources").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [←] ", theme.nav()),
            Span::raw("- Focus Planets Panel").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [→] ", theme.nav()),
            Span::raw("- Focus Explorers Panel").default(theme),
        ]),
    ]);
    text.push(Line::from(""));
    text.push(Line::from("")); // Separator

    // --- 3. COSMIC ACTIONS ---
    text.push(Line::from(vec![Span::styled(
        " Cosmic Actions ",
        theme.accent().bold(),
    )]));
    text.extend([
        Line::from(vec![
            Span::styled(" [I/K] ", theme.key()),
            Span::raw("- Increase / Decrease Sunray % (pause first)").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [S] ", theme.key()),
            Span::raw("- Send Sunray (Select Planet First)").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [A] ", theme.key()),
            Span::raw("- Send Asteroid (Select Planet First)").default(theme),
        ]),
    ]);
    text.push(Line::from(""));
    text.push(Line::from("")); // Separator

    // --- 4. CONTEXT / EXPLORERS ---
    text.push(Line::from(vec![Span::styled(
        " Manual Actions ",
        theme.accent().bold(),
    )]));
    text.extend([
        Line::from(vec![
            Span::styled(" [M] ", theme.key()),
            Span::raw("- Move selected Explorer").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [G] ", theme.key()),
            Span::raw("- Generate Resource from Explorer").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [ENTER] ", theme.key()),
            Span::raw("- Confirm Selection / Move / Resource").default(theme),
        ]),
        Line::from(vec![
            Span::styled(" [ESC] ", theme.key()),
            Span::raw("- Abort current action").default(theme),
        ]),
    ]);

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(theme.default()),
    );
    frame.render_widget(paragraph, area);
}
