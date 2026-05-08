mod colors;
mod ext;

use ratatui::style::{Modifier, Style};

use crate::game_state::GameState;

pub use colors::Palette;
pub use ext::{BlockThemeExt, SpanThemeExt};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub palette: Palette,
}

impl Theme {
    pub fn matrix() -> Self {
        Self {
            palette: Palette::matrix(),
        }
    }

    pub fn muted(&self) -> Style {
        Style::default().fg(self.palette.fg_muted)
    }

    pub fn value(&self) -> Style {
        Style::default().fg(self.palette.fg_primary)
    }

    pub fn accent(&self) -> Style {
        Style::default().fg(self.palette.fg_accent)
    }

    pub fn warning(&self) -> Style {
        Style::default().fg(self.palette.fg_warning)
    }

    pub fn danger(&self) -> Style {
        Style::default()
            .fg(self.palette.fg_danger)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success(&self) -> Style {
        Style::default().fg(self.palette.fg_success)
    }

    pub fn key(&self) -> Style {
        Style::default().fg(self.palette.fg_key).add_modifier(Modifier::BOLD)
    }

    pub fn nav(&self) -> Style {
        Style::default().fg(self.palette.fg_nav).add_modifier(Modifier::BOLD)
    }

    pub fn header(&self) -> Style {
        Style::default()
            .fg(self.palette.fg_warning)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border(&self) -> Style {
        Style::default().fg(self.palette.border)
    }

    pub fn border_active(&self) -> Style {
        Style::default().fg(self.palette.border_active)
    }

    pub fn row_highlight(&self) -> Style {
        Style::default()
            .bg(self.palette.bg_highlight)
            .fg(self.palette.fg_primary)
    }

    pub fn game_state_style(&self, state: GameState) -> Style {
        match state {
            GameState::Running => self.danger(),
            GameState::Paused => self.warning().add_modifier(Modifier::BOLD),
            GameState::Ended => Style::default()
                .fg(self.palette.fg_info)
                .add_modifier(Modifier::BOLD),
            GameState::WaitingStart => Style::default()
                .fg(self.palette.fg_info_dim)
                .add_modifier(Modifier::BOLD),
        }
    }
}
