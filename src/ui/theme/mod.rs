mod colors;
mod ext;

use ratatui::style::{Modifier, Style};

use crate::game_state::GameState;

pub use colors::Palette;
pub use ext::{BlockThemeExt, SpanThemeExt};

/// UI theme context.
///
/// Holds palette and exposes intent-based `Style` presets used by render code.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Raw color palette used by this theme instance.
    pub palette: Palette,
}

impl Theme {
    /// Default theme instance used by app.
    pub fn matrix() -> Self {
        Self {
            palette: Palette::matrix(),
        }
    }

    /// Style for de-emphasized labels and separators.
    pub fn muted(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for main body values.
    pub fn value(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for accents and callouts.
    pub fn accent(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for warning states.
    pub fn warning(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for danger or error states.
    pub fn danger(&self) -> Style {
        Style::default()
            .fg(self.palette.fg_danger)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for success states.
    pub fn success(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for key hints and shortcuts.
    pub fn key(&self) -> Style {
        Style::default()
            .fg(self.palette.matrix_green)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for navigation hints.
    pub fn nav(&self) -> Style {
        Style::default()
            .fg(self.palette.matrix_green)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for table headers.
    pub fn header(&self) -> Style {
        Style::default()
            .fg(self.palette.fg_warning)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for normal panel borders.
    pub fn border(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for active or focused panel borders.
    pub fn border_active(&self) -> Style {
        Style::default().fg(self.palette.matrix_green)
    }

    /// Style for highlighted rows (selection).
    pub fn row_highlight(&self) -> Style {
        Style::default()
            .bg(self.palette.bg_highlight)
            .fg(self.palette.matrix_green)
    }

    /// Style mapping for game state indicator in global header.
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
