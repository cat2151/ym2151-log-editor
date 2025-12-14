use crate::models::Ym2151Log;

/// Display mode for time values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeDisplayMode {
    /// Display cumulative time (delta from previous event)
    Cumulative,
    /// Display absolute timestamp
    Timestamp,
}

/// Application state
pub struct App {
    /// The loaded YM2151 log data
    pub log: Ym2151Log,
    /// Current file path (if any)
    pub file_path: Option<String>,
    /// Current scroll position
    pub scroll_offset: usize,
    /// Time display mode
    pub time_mode: TimeDisplayMode,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Selected event index
    pub selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            log: Ym2151Log { events: vec![] },
            file_path: None,
            scroll_offset: 0,
            time_mode: TimeDisplayMode::Cumulative,
            should_quit: false,
            selected_index: 0,
        }
    }

    /// Load a JSON file
    pub fn load_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        self.log = serde_json::from_str(&content)?;
        self.file_path = Some(path.to_string());
        self.selected_index = 0;
        self.scroll_offset = 0;
        Ok(())
    }

    /// Save the current log to file
    pub fn save_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.file_path {
            let content = serde_json::to_string_pretty(&self.log)?;
            std::fs::write(path, content)?;
            Ok(())
        } else {
            Err("No file path set".into())
        }
    }

    /// Toggle time display mode
    pub fn toggle_time_mode(&mut self) {
        self.time_mode = match self.time_mode {
            TimeDisplayMode::Cumulative => TimeDisplayMode::Timestamp,
            TimeDisplayMode::Timestamp => TimeDisplayMode::Cumulative,
        };
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
    pub fn move_down(&mut self) {
        if self.selected_index < self.log.events.len().saturating_sub(1) {
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

    /// Get cumulative time for an event (delta from previous)
    pub fn get_cumulative_time(&self, index: usize) -> f64 {
        if index == 0 {
            self.log.events[0].time
        } else if index < self.log.events.len() {
            self.log.events[index].time - self.log.events[index - 1].time
        } else {
            0.0
        }
    }

    /// Get formatted time string for an event
    pub fn get_time_string(&self, index: usize) -> String {
        if index >= self.log.events.len() {
            return String::from("0.000000");
        }

        let time = match self.time_mode {
            TimeDisplayMode::Timestamp => self.log.events[index].time,
            TimeDisplayMode::Cumulative => self.get_cumulative_time(index),
        };

        format!("{:.6}", time)
    }

    /// Format event for display
    pub fn format_event(&self, index: usize) -> String {
        if index >= self.log.events.len() {
            return String::new();
        }

        let event = &self.log.events[index];
        let time_str = self.get_time_string(index);

        if event.is_key_on() {
            format!("{}  KeyON  {}", time_str, event.data)
        } else {
            format!("{}  {}  {}", time_str, event.addr, event.data)
        }
    }

    /// Preview current event by playing events from start up to selected position
    #[cfg(windows)]
    pub fn preview_current_event(&self) {
        if self.log.events.is_empty() {
            return;
        }

        // Create a log containing events from start to current selection (inclusive)
        let end_index = self
            .selected_index
            .saturating_add(1)
            .min(self.log.events.len());
        let preview_events = self.log.events[0..end_index].to_vec();
        let preview_log = crate::models::Ym2151Log {
            events: preview_events,
        };

        // Convert to JSON and send to server
        if let Ok(json_string) = serde_json::to_string(&preview_log) {
            if let Err(e) = ym2151_log_play_server::client::send_json(&json_string) {
                eprintln!("Preview playback error: {}", e);
            }
        }
    }

    #[cfg(not(windows))]
    pub fn preview_current_event(&self) {
        // No-op on non-Windows platforms
    }

    /// Set wait time (cumulative time) for the selected event in milliseconds
    /// Only works in Cumulative display mode
    ///
    /// # Arguments
    /// * `milliseconds` - The wait time in milliseconds (typically 0-9).
    ///   Values are used as-is without validation. Common usage:
    ///   0-9ms (mapped from keys 0-9).
    pub fn set_wait_time_ms(&mut self, milliseconds: u32) {
        // Only allow modification in Cumulative mode
        if self.time_mode != TimeDisplayMode::Cumulative {
            return;
        }

        // Check if we have events and a valid selection
        if self.log.events.is_empty() || self.selected_index >= self.log.events.len() {
            return;
        }

        // Convert milliseconds to seconds
        let new_wait_time = (milliseconds as f64) / 1000.0;

        // Calculate the new absolute timestamp for the selected event
        let new_timestamp = if self.selected_index == 0 {
            // First event: set absolute time
            new_wait_time
        } else {
            // Other events: add wait time to previous event's timestamp
            self.log.events[self.selected_index - 1].time + new_wait_time
        };

        // Calculate the time delta (how much we're changing)
        let old_timestamp = self.log.events[self.selected_index].time;
        let time_delta = new_timestamp - old_timestamp;

        // Update the selected event's timestamp
        self.log.events[self.selected_index].time = new_timestamp;

        // Adjust all subsequent events' timestamps by the same delta
        for i in (self.selected_index + 1)..self.log.events.len() {
            self.log.events[i].time += time_delta;
        }
    }

    /// Delete the currently selected event
    pub fn delete_selected_event(&mut self) {
        // Check if we have events and a valid selection
        if self.log.events.is_empty() || self.selected_index >= self.log.events.len() {
            return;
        }

        // Remove the selected event
        self.log.events.remove(self.selected_index);

        // Adjust selected_index if it's now out of bounds
        if !self.log.events.is_empty() && self.selected_index >= self.log.events.len() {
            self.selected_index = self.log.events.len() - 1;
        }

        // Adjust scroll_offset if necessary
        if self.scroll_offset > self.selected_index {
            self.scroll_offset = self.selected_index;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ym2151Event;

    #[test]
    fn test_set_wait_time_ms() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Cumulative;

        // Create test events
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Select event 1 and set wait time to 5ms
        app.selected_index = 1;
        app.set_wait_time_ms(5);

        // Verify event 1 now has timestamp 0.005 (0.0 + 0.005)
        assert!((app.log.events[1].time - 0.005).abs() < 0.0001);

        // Verify event 2 was also adjusted (should be 0.015, was 0.02, delta = -0.005)
        assert!((app.log.events[2].time - 0.015).abs() < 0.0001);
    }

    #[test]
    fn test_set_wait_time_ms_timestamp_mode() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Timestamp;

        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        app.selected_index = 1;
        let original_time = app.log.events[1].time;

        // Should not modify in Timestamp mode
        app.set_wait_time_ms(5);

        assert_eq!(app.log.events[1].time, original_time);
    }

    #[test]
    fn test_set_wait_time_ms_first_event() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Cumulative;

        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Select first event and set wait time to 3ms
        app.selected_index = 0;
        app.set_wait_time_ms(3);

        // First event should be at 0.003
        assert!((app.log.events[0].time - 0.003).abs() < 0.0001);

        // Second event should also be adjusted (was 0.01, delta = +0.003)
        assert!((app.log.events[1].time - 0.013).abs() < 0.0001);
    }

    #[test]
    fn test_set_wait_time_ms_zero() {
        let mut app = App::new();
        app.time_mode = TimeDisplayMode::Cumulative;

        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Select event 1 and set wait time to 0ms
        app.selected_index = 1;
        app.set_wait_time_ms(0);

        // Verify event 1 now has timestamp 0.0 (same as previous event)
        assert!((app.log.events[1].time - 0.0).abs() < 0.0001);

        // Verify event 2 was also adjusted (should be 0.01, was 0.02, delta = -0.01)
        assert!((app.log.events[2].time - 0.01).abs() < 0.0001);
    }

    #[test]
    fn test_delete_selected_event() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
            Ym2151Event {
                time: 0.02,
                addr: "60".to_string(),
                data: "14".to_string(),
            },
        ];

        // Select middle event and delete it
        app.selected_index = 1;
        app.delete_selected_event();

        // Verify event count decreased
        assert_eq!(app.log.events.len(), 2);

        // Verify the correct event was deleted (remaining events should be index 0 and 2)
        assert_eq!(app.log.events[0].addr, "20");
        assert_eq!(app.log.events[1].addr, "60");

        // Verify selected_index is still valid
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn test_delete_last_event() {
        let mut app = App::new();
        app.log.events = vec![
            Ym2151Event {
                time: 0.0,
                addr: "20".to_string(),
                data: "4F".to_string(),
            },
            Ym2151Event {
                time: 0.01,
                addr: "40".to_string(),
                data: "16".to_string(),
            },
        ];

        // Select last event and delete it
        app.selected_index = 1;
        app.delete_selected_event();

        // Verify event count decreased
        assert_eq!(app.log.events.len(), 1);

        // Verify selected_index was adjusted to last valid index
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_delete_single_event() {
        let mut app = App::new();
        app.log.events = vec![Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        }];

        // Select the only event and delete it
        app.selected_index = 0;
        app.delete_selected_event();

        // Verify all events are deleted
        assert_eq!(app.log.events.len(), 0);

        // selected_index should remain 0 (though there are no events)
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_delete_empty_list() {
        let mut app = App::new();
        app.log.events = vec![];

        // Try to delete from empty list (should not panic)
        app.selected_index = 0;
        app.delete_selected_event();

        // Verify still empty
        assert_eq!(app.log.events.len(), 0);
    }
}
