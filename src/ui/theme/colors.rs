use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub fg_primary: Color,
    pub fg_muted: Color,
    pub fg_accent: Color,
    pub fg_warning: Color,
    pub fg_danger: Color,
    pub fg_success: Color,
    pub fg_info: Color,
    pub fg_info_dim: Color,
    pub fg_key: Color,
    pub fg_nav: Color,

    pub border: Color,
    pub border_active: Color,

    pub bg_highlight: Color,
}

impl Palette {
    pub fn matrix() -> Self {
        Self {
            fg_primary: Color::White,
            fg_muted: Color::Gray,
            fg_accent: Color::Cyan,
            fg_warning: Color::Yellow,
            fg_danger: Color::Red,
            fg_success: Color::Green,
            fg_info: Color::Blue,
            fg_info_dim: Color::LightCyan,
            fg_key: Color::Magenta,
            fg_nav: Color::Cyan,
            border: Color::DarkGray,
            border_active: Color::LightRed,
            bg_highlight: Color::DarkGray,
        }
    }
}
