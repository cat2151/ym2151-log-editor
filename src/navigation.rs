/// Navigation state for event list
#[derive(Debug, Clone)]
pub struct NavigationState {
    /// Current scroll position
    pub scroll_offset: usize,
    /// Selected event index
    pub selected_index: usize,
}

impl NavigationState {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            selected_index: 0,
        }
    }

    /// Reset navigation to the beginning
    pub fn reset(&mut self) {
        self.scroll_offset = 0;
        self.selected_index = 0;
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }

    /// Move selection down
    /// Allow cursor to move to one position beyond the last event (for future insertion)
    pub fn move_down(&mut self, max_events: usize) {
        if self.selected_index < max_events {
            self.selected_index += 1;
        }
    }

    /// Update scroll offset to keep selected item visible
    pub fn update_scroll(&mut self, visible_height: usize) {
        if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }

    /// Adjust navigation after an event is deleted
    pub fn adjust_after_delete(&mut self, events_count: usize) {
        // Adjust selected_index if it's now out of bounds
        if events_count > 0 && self.selected_index >= events_count {
            self.selected_index = events_count - 1;
        }

        // Adjust scroll_offset if necessary
        if self.scroll_offset > self.selected_index {
            self.scroll_offset = self.selected_index;
        }
    }

    /// Adjust navigation after an event is inserted
    pub fn adjust_after_insert(&mut self) {
        // Adjust scroll_offset if necessary to keep the new event visible
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}
