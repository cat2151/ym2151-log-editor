use crate::models::Ym2151Log;
use crate::navigation::NavigationState;
use crate::time_display::TimeDisplayMode;
use std::time::Instant;

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
    /// Whether loop playback is enabled
    pub loop_enabled: bool,
    /// Last time a loop iteration started
    loop_started_at: Option<Instant>,
    /// Whether loop playback should be refreshed on next tick
    loop_dirty: bool,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Whether the help overlay is visible
    help_visible: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            log: Ym2151Log { events: vec![] },
            file_path: None,
            navigation: NavigationState::new(),
            time_mode: TimeDisplayMode::Cumulative,
            loop_enabled: false,
            loop_started_at: None,
            loop_dirty: false,
            should_quit: false,
            help_visible: false,
        }
    }

    /// Load a JSON file
    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.disable_loop();
        self.log = crate::file_io::load_file(path)?;
        self.file_path = Some(path.to_string());
        self.navigation.reset();
        Ok(())
    }

    /// Load from a JSON string (e.g. clipboard content)
    pub fn load_from_str(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.disable_loop();
        self.log = serde_json::from_str(content)?;
        self.file_path = None;
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

    /// Toggle loop playback using interactive mode
    pub fn toggle_loop(&mut self) {
        if self.loop_enabled {
            self.disable_loop();
            return;
        }

        if self.log.events.is_empty() || self.loop_duration_seconds() <= 0.0 {
            self.disable_loop();
            return;
        }

        if crate::preview::start_loop_playback(&self.log) {
            self.loop_enabled = true;
            self.loop_started_at = Some(Instant::now());
            self.loop_dirty = false;
        }
    }

    /// Periodic tick to manage loop playback scheduling
    pub fn tick(&mut self) {
        if !self.loop_enabled {
            return;
        }

        if self.log.events.is_empty() {
            self.disable_loop();
            return;
        }

        let duration = self.loop_duration_seconds();
        if duration <= 0.0 {
            self.disable_loop();
            return;
        }

        let should_restart = self.loop_dirty
            || self
                .loop_started_at
                .map(|t| t.elapsed().as_secs_f64() >= duration)
                .unwrap_or(true);

        if should_restart && crate::preview::queue_loop_playback(&self.log) {
            self.loop_started_at = Some(Instant::now());
            self.loop_dirty = false;
        }
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

        self.mark_loop_dirty();
    }

    /// Delete the currently selected event
    pub fn delete_selected_event(&mut self) {
        crate::event_editor::delete_event(&mut self.log, self.navigation.selected_index);
        self.navigation.adjust_after_delete(self.log.events.len());
        self.mark_loop_dirty();
    }

    /// Insert a new event before the currently selected position
    pub fn insert_event_before_selected(&mut self) {
        crate::event_editor::insert_event_before(&mut self.log, self.navigation.selected_index);
        self.navigation.adjust_after_insert();
        self.mark_loop_dirty();
    }

    // Accessor methods for backward compatibility with UI code
    pub fn selected_index(&self) -> usize {
        self.navigation.selected_index
    }

    pub fn scroll_offset(&self) -> usize {
        self.navigation.scroll_offset
    }

    pub fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    pub fn hide_help(&mut self) {
        self.help_visible = false;
    }

    pub fn help_visible(&self) -> bool {
        self.help_visible
    }

    fn disable_loop(&mut self) {
        crate::preview::stop_loop_playback();
        self.loop_enabled = false;
        self.loop_started_at = None;
        self.loop_dirty = false;
    }

    fn loop_duration_seconds(&self) -> f64 {
        self.log.events.last().map(|e| e.time).unwrap_or(0.0)
    }

    fn mark_loop_dirty(&mut self) {
        if self.loop_enabled {
            self.loop_dirty = true;
            self.loop_started_at = None;
        }
    }

    #[cfg(test)]
    pub(crate) fn test_set_loop_started_at(&mut self, instant: Instant) {
        self.loop_started_at = Some(instant);
    }

    #[cfg(test)]
    pub(crate) fn test_set_loop_dirty(&mut self, dirty: bool) {
        self.loop_dirty = dirty;
    }

    #[cfg(test)]
    pub(crate) fn test_loop_started_at(&self) -> Option<Instant> {
        self.loop_started_at
    }

    #[cfg(test)]
    pub(crate) fn test_loop_dirty(&self) -> bool {
        self.loop_dirty
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
