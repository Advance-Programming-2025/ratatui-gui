use ratatui::style::Color;

/// Primary brand color used by current UI theme.
pub const MATRIX_GREEN: Color = Color::Rgb(51, 255, 51);

/// Theme palette containing raw colors for semantic roles.
///
/// Theme layer selects which roles are used for each widget style.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    /// Matrix brand green used by current layout styling.
    pub matrix_green: Color,
    /// Foreground for warnings and attention-grabbing elements.
    pub fg_warning: Color,
    /// Foreground for error or danger states.
    pub fg_danger: Color,
    //Background for selected instances
    pub bg_highlight: Color,
}

impl Palette {
    /// Default "Matrix" palette.
    pub(crate) fn matrix() -> Self {
        Self {
            matrix_green: MATRIX_GREEN,
            fg_warning: Color::Yellow,
            fg_danger: Color::Red,
            bg_highlight: Color::DarkGray,
        }
    }
}
