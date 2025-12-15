use crate::models::Ym2151Log;

/// Preview current event by playing events from start up to selected position
#[cfg(windows)]
pub fn preview_current_event(log: &Ym2151Log, selected_index: usize) {
    if log.events.is_empty() {
        return;
    }

    // Create a log containing events from start to current selection (inclusive)
    let end_index = selected_index.saturating_add(1).min(log.events.len());
    let preview_events = log.events[0..end_index].to_vec();
    let preview_log = Ym2151Log {
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
pub fn preview_current_event(_log: &Ym2151Log, _selected_index: usize) {
    // No-op on non-Windows platforms
}
