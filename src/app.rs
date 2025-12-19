use crate::models::Ym2151Log;
use crate::navigation::NavigationState;
use crate::time_display::TimeDisplayMode;

/// Application state
pub struct App {
    /// The loaded YM2151 log data
    pub log: Ym2151Log,
    /// Current file path (if any)
    pub file_path: Option<String>,
    /// Navigation state (scroll and selection)
    pub navigation: NavigationState,
    /// Time display mode
    pub time_mode: TimeDisplayMode,
    /// Whether the app should quit
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            log: Ym2151Log { events: vec![] },
            file_path: None,
            navigation: NavigationState::new(),
            time_mode: TimeDisplayMode::Cumulative,
            should_quit: false,
        }
    }

    /// Load a JSON file
    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.log = crate::file_io::load_file(path)?;
        self.file_path = Some(path.to_string());
        self.navigation.reset();
        Ok(())
    }

    /// Save the current log to file
    pub fn save_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            crate::file_io::save_file(path, &self.log)?;
            Ok(())
        } else {
            Err("No file path set".into())
        }
    }

    /// Toggle time display mode
    pub fn toggle_time_mode(&mut self) {
        self.time_mode.toggle();
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        self.navigation.move_up();
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        self.navigation.move_down(self.log.events.len());
    }

    /// Update scroll offset to keep selected item visible
    pub fn update_scroll(&mut self, visible_height: usize) {
        self.navigation.update_scroll(visible_height);
    }

    /// Format event for display
    pub fn format_event(&self, index: usize) -> String {
        crate::time_display::format_event(&self.log, index, self.time_mode)
    }

    /// Preview playback by playing all events in the log
    pub fn preview_current_event(&self) {
        crate::preview::preview_current_event(&self.log, self.navigation.selected_index);
    }

    /// Set wait time (cumulative time) for the selected event in milliseconds
    /// Only works in Cumulative display mode
    ///
    /// # Arguments
    /// * `milliseconds` - The wait time in milliseconds (typically 0-9).
    ///   Values are used as-is without validation. Common usage:
    ///   0-9ms (mapped from keys 0-9).
    pub fn set_wait_time_ms(&mut self, milliseconds: u32) {
        crate::event_editor::set_wait_time_ms(
            &mut self.log,
            self.navigation.selected_index,
            milliseconds,
            self.time_mode,
        );
    }

    /// Delete the currently selected event
    pub fn delete_selected_event(&mut self) {
        crate::event_editor::delete_event(&mut self.log, self.navigation.selected_index);
        self.navigation.adjust_after_delete(self.log.events.len());
    }

    /// Insert a new event before the currently selected position
    pub fn insert_event_before_selected(&mut self) {
        crate::event_editor::insert_event_before(&mut self.log, self.navigation.selected_index);
        self.navigation.adjust_after_insert();
    }

    // Accessor methods for backward compatibility with UI code
    pub fn selected_index(&self) -> usize {
        self.navigation.selected_index
    }

    pub fn scroll_offset(&self) -> usize {
        self.navigation.scroll_offset
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
