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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
