use crate::models::{Ym2151Event, Ym2151Log};
use crate::time_display::TimeDisplayMode;

/// Set wait time (cumulative time) for the selected event in milliseconds
/// Only works in Cumulative display mode
///
/// # Arguments
/// * `log` - The YM2151 log to modify
/// * `selected_index` - Index of the event to modify
/// * `milliseconds` - The wait time in milliseconds (typically 0-9).
///   Values are used as-is without validation. Common usage:
///   0-9ms (mapped from keys 0-9).
/// * `time_mode` - Current time display mode (only works in Cumulative mode)
pub fn set_wait_time_ms(
    log: &mut Ym2151Log,
    selected_index: usize,
    milliseconds: u32,
    time_mode: TimeDisplayMode,
) {
    // Only allow modification in Cumulative mode
    if time_mode != TimeDisplayMode::Cumulative {
        return;
    }

    // Check if we have events and a valid selection
    if log.events.is_empty() || selected_index >= log.events.len() {
        return;
    }

    // Convert milliseconds to seconds
    let new_wait_time = (milliseconds as f64) / 1000.0;

    // Calculate the new absolute timestamp for the selected event
    let new_timestamp = if selected_index == 0 {
        // First event: set absolute time
        new_wait_time
    } else {
        // Other events: add wait time to previous event's timestamp
        log.events[selected_index - 1].time + new_wait_time
    };

    // Calculate the time delta (how much we're changing)
    let old_timestamp = log.events[selected_index].time;
    let time_delta = new_timestamp - old_timestamp;

    // Update the selected event's timestamp
    log.events[selected_index].time = new_timestamp;

    // Adjust all subsequent events' timestamps by the same delta
    for i in (selected_index + 1)..log.events.len() {
        log.events[i].time += time_delta;
    }
}

/// Delete the event at the specified index
pub fn delete_event(log: &mut Ym2151Log, selected_index: usize) {
    // Check if we have events and a valid selection
    if log.events.is_empty() || selected_index >= log.events.len() {
        return;
    }

    // Remove the selected event
    log.events.remove(selected_index);
}

/// Insert a new event before the specified position
pub fn insert_event_before(log: &mut Ym2151Log, selected_index: usize) {
    // Calculate the timestamp for the new event
    let new_time = if selected_index == 0 {
        // Inserting before the first event: use time 0.0
        0.0
    } else if selected_index >= log.events.len() {
        // Inserting after all events: use last event's time
        log.events.last().map(|e| e.time).unwrap_or(0.0)
    } else {
        // Inserting between events: use previous event's time
        log.events[selected_index - 1].time
    };

    // Create a new default event
    let new_event = Ym2151Event {
        time: new_time,
        addr: "00".to_string(),
        data: "00".to_string(),
    };

    // Insert the new event at the selected position
    // insert() can handle index == len(), which appends to the end
    log.events.insert(selected_index, new_event);
}
