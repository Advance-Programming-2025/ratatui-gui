//! Input normalization for UI controller.
//! Maps raw terminal key events into semantic actions.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

/// Semantic input actions used by controller state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Action {
    Quit,
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Cancel,
    TogglePause,
    ToggleLog,
    MoveMode,
    GenerateResource,
    SendAsteroid,
    SendSunray,
    Digit(u8),
    None,
}

/// Convert `KeyEvent` into an `Action`.
/// Filters non-press events to reduce input noise.
pub(crate) fn map_key(event: KeyEvent) -> Action {
    if event.kind != KeyEventKind::Press {
        return Action::None;
    }

    match event.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Up => Action::Up,
        KeyCode::Down => Action::Down,
        KeyCode::Left => Action::Left,
        KeyCode::Right => Action::Right,
        KeyCode::Enter => Action::Confirm,
        KeyCode::Esc => Action::Cancel,
        KeyCode::Char('p') => Action::TogglePause,
        KeyCode::Char('l') => Action::ToggleLog,
        KeyCode::Char('m') => Action::MoveMode,
        KeyCode::Char('g') => Action::GenerateResource,
        KeyCode::Char('a') => Action::SendAsteroid,
        KeyCode::Char('s') => Action::SendSunray,
        KeyCode::Char(c) if c.is_ascii_digit() => Action::Digit(c as u8 - b'0'),
        _ => Action::None,
    }
}
