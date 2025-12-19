use crate::models::Ym2151Log;

/// Preview playback by playing all events in the log
#[cfg(windows)]
pub fn preview_current_event(log: &Ym2151Log, _selected_index: usize) {
    if log.events.is_empty() {
        return;
    }

    // Convert entire log to JSON and send to server
    if let Ok(json_string) = serde_json::to_string(log) {
        if let Err(e) = ym2151_log_play_server::client::send_json(&json_string) {
            eprintln!("Preview playback error: {}", e);
        }
    }
}

#[cfg(not(windows))]
pub fn preview_current_event(_log: &Ym2151Log, _selected_index: usize) {
    // No-op on non-Windows platforms
}
