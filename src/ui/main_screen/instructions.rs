use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::app::App;
use crate::ui::theme::{SpanThemeExt, Theme};

/// Render help panel with key bindings.
pub(crate) fn render_instructions(_app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let mut text = vec![Line::from("")];

    if let Some(banner) = _app.ui.overlays.banner.as_ref() {
        text.push(Line::from(vec![
            Span::styled("  Notice: ", theme.warning().bold()),
            Span::styled(banner.as_str(), theme.value().bold()),
        ]));
        text.push(Line::from(""));
    }

    text.extend([
        Line::from(vec![
            Span::raw("  Q ").danger(theme),
            Span::raw("- Quit").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  P ", theme.warning().bold()),
            Span::raw("- Pause/Resume").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  ↑ ", theme.nav()),
            Span::raw("- Select up Explorer/Planet").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  ↓ ", theme.nav()),
            Span::raw("- Select down Explorer/Planet").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  ← ", theme.nav()),
            Span::raw("- Focus Planets").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  → ", theme.nav()),
            Span::raw("- Focus Explorers").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  L ", theme.success().bold()),
            Span::raw("- Toggle Log Overlay").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  O ", theme.key()),
            Span::raw("- Increase Sunray %(pause first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  I ", theme.key()),
            Span::raw("- Decrease Sunray %(pause first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  S ", theme.key()),
            Span::raw("- Send Sunray(select planet first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  A ", theme.key()),
            Span::raw("- Send Asteroid(select planet first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  M ", theme.key()),
            Span::raw("- Move Explorer(select explorer first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  G ", theme.key()),
            Span::raw("- Generate Resource(select explorer first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  ENTER ", theme.key()),
            Span::raw("- Confirm selection / move / resource").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  ESC ", theme.key()),
            Span::raw("- Abort move / resource").value(theme),
        ]),
    ]);

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(theme.warning()),
    );
    frame.render_widget(paragraph, area);
}
