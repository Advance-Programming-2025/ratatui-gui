//! Reusable selection cursors for stateful widgets.
//! Provides minimal row navigation without embedding domain logic.

use ratatui::widgets::{ListState, TableState};

/// Generic row selector used by tables and lists.
pub struct ListSelector{
    state: TableState,
    last_selected: Option<usize>,
}

impl ListSelector {
    /// Create empty selector with no selection.
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            last_selected: None,
        }
    }

    /// Current selected row index.
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
    

    /// Last non-`None` selection seen.
    pub fn last_selected(&self) -> Option<usize> {
        self.last_selected
    }

    /// Set last selected planet for explorer update
    pub(crate) fn set_last_selected(&mut self, explorer_planet:usize){
        self.last_selected = Some(explorer_planet);
    }

    /// Last selection as `u32` for id usage.
    pub fn last_selected_u32(&self) -> Option<u32> {
        self.last_selected.map(|i| i as u32)
    }

    /// Clear selection, keep last selection for restoring later.
    pub fn clear(&mut self) {
        self.state.select(None);
    }

    /// Restore last selection if valid, otherwise select last row.
    pub fn restore_last(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        if let Some(i) = self.last_selected {
            if i < len {
                self.state.select(Some(i));
                return;
            }
        }

        self.select_first(len);
    }

    /// Select first row if list non-empty.
    pub fn select_first(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        self.state.select(Some(0));
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Move selection up within bounds.
    pub fn move_up(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        match self.state.selected() {
            Some(i) if i > 0 => self.state.select(Some(i - 1)),
            Some(_) => {}
            None => self.state.select(Some(0)),
        }
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Move selection down within bounds.
    pub fn move_down(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        match self.state.selected() {
            Some(i) if i + 1 < len => self.state.select(Some(i + 1)),
            Some(_) => {}
            None => self.state.select(Some(0)),
        }
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Mutable `TableState` for `render_stateful_widget`.
    pub fn state_mut(&mut self) -> &mut TableState {
        &mut self.state
    }
}

/// Generic row selector used by list widgets.
#[derive(Debug, Clone)]
pub struct ResourceSelector {
    state: ListState,
    last_selected: Option<usize>,
}

impl ResourceSelector {
    /// Create empty selector with no selection.
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            last_selected: None,
        }
    }

    /// Current selected row index.
    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    /// Last non-`None` selection seen.
    pub fn last_selected(&self) -> Option<usize> {
        self.last_selected
    }

    /// Clear selection, keep last selection for restoring later.
    pub fn clear(&mut self) {
        self.state.select(None);
    }

    /// Restore last selection if valid, otherwise select first row.
    pub fn restore_last(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        if let Some(i) = self.last_selected {
            if i < len {
                self.state.select(Some(i));
                return;
            }
        }

        self.select_first(len);
    }

    /// Select first row if list non-empty.
    pub fn select_first(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        self.state.select(Some(0));
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Select last row if list non-empty.
    pub fn select_last(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        self.state.select(Some(len - 1));
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Move selection up within bounds.
    pub fn move_up(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        match self.state.selected() {
            Some(i) if i > 0 => self.state.select(Some(i - 1)),
            Some(_) => {}
            None => self.state.select(Some(0)),
        }
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Move selection down within bounds.
    pub fn move_down(&mut self, len: usize) {
        if len == 0 {
            self.clear();
            return;
        }

        match self.state.selected() {
            Some(i) if i + 1 < len => self.state.select(Some(i + 1)),
            Some(_) => {}
            None => self.state.select(Some(0)),
        }
        self.last_selected = self.state.selected().or(self.last_selected);
    }

    /// Mutable `ListState` for `render_stateful_widget`.
    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }
}

/// All UI selectors owned by `AppUi`.
pub struct Selectors {
    pub planets: ListSelector,
    pub explorers: ListSelector,
    pub resources: ResourceSelector,
}

impl Selectors {
    /// Create selectors for all lists.
    pub(crate) fn new() -> Self {
        Self {
            planets: ListSelector::new(),
            explorers: ListSelector::new(),
            resources: ResourceSelector::new(),
        }
    }
}
