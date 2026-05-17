//! UI-only state and nested interaction modes.
//! Owns focus, selectors, overlays, and modal flows.

use crate::selector::Selectors;

/// Which list receives navigation input in normal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Planets,
    Explorers,
}

/// Nested UI interaction mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiMode {
    /// Default navigation.
    Normal,
    /// Modal "select planet destination" flow.
    MoveExplorer {
        explorer_id: u32,
        return_focus: Focus,
    },
}

/// Start screen UI state.
#[derive(Debug, Clone)]
pub struct StartScreenState {
    /// 0 = Random, 1 = Custom.
    pub selected_mode: u8,
    /// Custom planets count when custom mode selected.
    pub custom_planet_count: u32,
}

impl Default for StartScreenState {
    fn default() -> Self {
        Self {
            selected_mode: 0,
            custom_planet_count: 0,
        }
    }
}

/// Overlay toggles and UI flags.
#[derive(Debug, Clone)]
pub struct OverlayState {
    pub show_log: bool,
    /// Optional banner line shown in UI.
    pub banner: Option<String>,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            show_log: false,
            banner: None,
        }
    }
}

/// UI state owned by app, separate from game model.
#[derive(Debug, Clone)]
pub struct AppUi {
    pub focus: Focus,
    pub mode: UiMode,
    pub selectors: Selectors,
    pub start: StartScreenState,
    pub overlays: OverlayState,
}

impl AppUi {
    /// Create initial UI state with empty selectors.
    pub fn new() -> Self {
        Self {
            focus: Focus::Planets,
            mode: UiMode::Normal,
            selectors: Selectors::new(),
            start: StartScreenState::default(),
            overlays: OverlayState::default(),
        }
    }
}

