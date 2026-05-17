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
    /// Main foreground for body text.
    // pub fg_primary: Color,
    /// Foreground for de-emphasized labels and separators.
    // pub fg_muted: Color,
    /// Foreground for accented UI elements.
    // pub fg_accent: Color,
    /// Foreground for warnings and attention-grabbing elements.
    pub fg_warning: Color,
    /// Foreground for error or danger states.
    pub fg_danger: Color,
    /// Foreground for success states.
    // pub fg_success: Color,
    /// Foreground for informational states.
    // pub fg_info: Color,
    /// Dimmer informational foreground for less prominent info.
    // pub fg_info_dim: Color,
    /// Foreground for key hints and shortcuts.
    // pub fg_key: Color,
    /// Foreground for navigation hints (arrows, selection guidance).
    // pub fg_nav: Color,

    /// Default border color for panels.
    // pub border: Color,
    /// Border color for active or focused panels.
    // pub border_active: Color,

    /// Background color for selection highlights.
    pub bg_highlight: Color,
}

impl Palette {
    /// Default "Matrix" palette.
    pub fn matrix() -> Self {
        Self {
            matrix_green: MATRIX_GREEN,

            // fg_primary: Color::White,
            // fg_muted: Color::Gray,
            // fg_accent: Color::Cyan,
            fg_warning: Color::Yellow,
            fg_danger: Color::Red,
            // fg_success: Color::Green,
            // fg_info: Color::Blue,
            // fg_info_dim: Color::LightCyan,
            // fg_key: Color::Magenta,
            // fg_nav: Color::Cyan,

            // border: Color::DarkGray,
            // border_active: Color::LightRed,

            bg_highlight: Color::DarkGray,
        }
    }
}
