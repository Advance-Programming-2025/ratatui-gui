//! Reusable selection cursors for stateful widgets.
//! Provides minimal row navigation without embedding domain logic.

use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use ratatui::widgets::{ListState, TableState};

use crate::app::App;

/// Generic row selector used by tables and lists.
pub struct ListSelector {
    state: TableState,
    last_selected: Option<usize>,
}

enum ListFocus {
    BasicList,
    ComplexList,
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
    pub(crate) fn set_last_selected(&mut self, explorer_planet: usize) {
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
pub struct ResourceSelector<T> {
    state: ListState,
    last_selected: Option<usize>,
    original_list: Vec<T>,
}

impl<T: Clone> ResourceSelector<T> {
    /// Create empty selector with no selection.
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            last_selected: None,
            original_list: vec![],
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.original_list.len()
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
    pub fn restore_last(&mut self) {
        let len = self.original_list.len();
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
    pub fn move_up(&mut self) {
        let len = self.original_list.len();
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
    pub fn move_down(&mut self) {
        let len = self.original_list.len();
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
    pub(crate) fn set_original_list(&mut self, new_list: Vec<T>) {
        self.original_list = new_list;
    }
    pub(crate) fn get_element_from_original_list(&mut self, index: usize) -> T {
        self.original_list[index].clone()
    }
}

/// All UI selectors owned by `AppUi`.
pub struct Selectors {
    pub planets: ListSelector,
    pub explorers: ListSelector,
    pub basic_resources: ResourceSelector<BasicResourceType>,
    pub complex_resource: ResourceSelector<ComplexResourceType>,
    pub list_focus: ListFocus,
}

impl Selectors {
    /// Create selectors for all lists.
    pub(crate) fn new() -> Self {
        Self {
            planets: ListSelector::new(),
            explorers: ListSelector::new(),
            basic_resources: ResourceSelector::new(),
            complex_resource: ResourceSelector::new(),
            list_focus: ListFocus::BasicList,
        }
    }

    pub(crate) fn list_resource_move_up(&mut self) {
        match self.list_focus {
            ListFocus::BasicList => {
                self.basic_resources.move_up();
            }
            ListFocus::ComplexList => self.complex_resource.move_up(),
        }
    }

    pub(crate) fn list_resource_move_down(&mut self) {
        match self.list_focus {
            ListFocus::BasicList => {
                self.basic_resources.move_down();
            }
            ListFocus::ComplexList => self.complex_resource.move_down(),
        }
    }
    pub(crate) fn list_resource_move_right(&mut self) {
        if self.complex_resource.len() != 0 {
            self.basic_resources.clear();
            self.complex_resource.restore_last();
            self.list_focus = ListFocus::ComplexList;
        }
    }
    pub(crate) fn list_resource_move_left(&mut self) {
        self.complex_resource.clear();
        self.basic_resources.restore_last();
        self.list_focus = ListFocus::BasicList;
    }
}
