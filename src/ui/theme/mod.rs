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
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn value(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn accent(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn warning(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn danger(&self) -> Style {
        Style::default()
            .fg(self.palette.matrix_green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn key(&self) -> Style {
        Style::default()
            .fg(self.palette.matrix_green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn nav(&self) -> Style {
        Style::default()
            .fg(self.palette.matrix_green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn header(&self) -> Style {
        Style::default()
            .fg(self.palette.fg_warning)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn border_active(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    pub fn row_highlight(&self) -> Style {
        Style::default()
            .bg(self.palette.bg_highlight)
            .fg(self.palette.matrix_green)
    }

    pub fn game_state_style(&self, state: GameState) -> Style {
        match state {
            GameState::Running => self.danger(),
            GameState::Paused => self.warning().add_modifier(Modifier::BOLD),
            GameState::Ended => Style::default()
                .fg(self.palette.matrix_green)
                .add_modifier(Modifier::BOLD),
            GameState::WaitingStart => Style::default()
                .fg(self.palette.matrix_green)
                .add_modifier(Modifier::BOLD),
        }
    }
}
