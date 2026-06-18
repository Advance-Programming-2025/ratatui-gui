use ratatui::{text::Span, widgets::Block};

use super::Theme;

/// Span helpers for applying theme intent styles with minimal boilerplate.
pub trait SpanThemeExt<'a> {
    /// Apply muted label style.
    fn default(self, theme: &Theme) -> Span<'a>;
}

impl<'a> SpanThemeExt<'a> for Span<'a> {
    fn default(self, theme: &Theme) -> Span<'a> {
        Self::styled(self.content, theme.default())
    }
}

pub trait BlockThemeExt<'a> {
    /// Apply standard panel border style.
    fn panel(self, theme: &Theme) -> Block<'a>;
    /// Apply active panel border style.
    fn panel_active(self, theme: &Theme) -> Block<'a>;
}

impl<'a> BlockThemeExt<'a> for Block<'a> {
    fn panel(self, theme: &Theme) -> Block<'a> {
        self.border_style(theme.border())
    }

    fn panel_active(self, theme: &Theme) -> Block<'a> {
        self.border_style(theme.border_active())
    }
}
