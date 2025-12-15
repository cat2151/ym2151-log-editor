use crate::models::Ym2151Log;

/// Display mode for time values
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeDisplayMode {
    /// Display cumulative time (delta from previous event)
    Cumulative,
    /// Display absolute timestamp
    Timestamp,
}

impl TimeDisplayMode {
    /// Toggle between display modes
    pub fn toggle(&mut self) {
        *self = match *self {
            TimeDisplayMode::Cumulative => TimeDisplayMode::Timestamp,
            TimeDisplayMode::Timestamp => TimeDisplayMode::Cumulative,
        };
    }
}

/// Get cumulative time for an event (delta from previous)
pub fn get_cumulative_time(log: &Ym2151Log, index: usize) -> f64 {
    if index == 0 {
        log.events[0].time
    } else if index < log.events.len() {
        log.events[index].time - log.events[index - 1].time
    } else {
        0.0
    }
}

/// Get formatted time string for an event
pub fn get_time_string(log: &Ym2151Log, index: usize, mode: TimeDisplayMode) -> String {
    if index >= log.events.len() {
        return String::from("0.000000");
    }

    let time = match mode {
        TimeDisplayMode::Timestamp => log.events[index].time,
        TimeDisplayMode::Cumulative => get_cumulative_time(log, index),
    };

    format!("{:.6}", time)
}

/// Format event for display
pub fn format_event(log: &Ym2151Log, index: usize, mode: TimeDisplayMode) -> String {
    if index >= log.events.len() {
        return String::new();
    }

    let event = &log.events[index];
    let time_str = get_time_string(log, index, mode);

    if event.is_key_on() {
        format!("{}  KeyON  {}", time_str, event.data)
    } else {
        format!("{}  {}  {}", time_str, event.addr, event.data)
    }
}
