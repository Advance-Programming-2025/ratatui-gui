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
    let text = vec![
        Line::from(""),
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
            Span::styled("  1..9 ", theme.key()),
            Span::raw("- Type Destination Planet ID(select explorer first)").value(theme),
        ]),
        Line::from(vec![
            Span::styled("  ENTER ", theme.key()),
            Span::raw("- Confirm Destination Planet ID").value(theme),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(theme.warning()),
    );
    frame.render_widget(paragraph, area);
}
