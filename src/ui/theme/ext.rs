use ratatui::{text::Span, widgets::Block};

use super::Theme;

pub trait SpanThemeExt<'a> {
    fn muted(self, theme: &Theme) -> Span<'a>;
    fn accent(self, theme: &Theme) -> Span<'a>;
    fn danger(self, theme: &Theme) -> Span<'a>;
    fn value(self, theme: &Theme) -> Span<'a>;
}

impl<'a> SpanThemeExt<'a> for Span<'a> {
    fn muted(self, theme: &Theme) -> Span<'a> {
        Self::styled(self.content, theme.muted())
    }

    fn accent(self, theme: &Theme) -> Span<'a> {
        Self::styled(self.content, theme.accent())
    }

    fn danger(self, theme: &Theme) -> Span<'a> {
        Self::styled(self.content, theme.danger())
    }

    fn value(self, theme: &Theme) -> Span<'a> {
        Self::styled(self.content, theme.value())
    }
}

pub trait BlockThemeExt<'a> {
    fn panel(self, theme: &Theme) -> Block<'a>;
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
