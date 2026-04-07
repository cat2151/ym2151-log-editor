use crate::models::Ym2151Log;

#[cfg(all(windows, not(test)))]
use std::sync::atomic::{AtomicBool, Ordering};

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

/// Start loop playback using interactive mode.
#[cfg(all(windows, not(test)))]
pub fn start_loop_playback(log: &Ym2151Log) -> bool {
    if log.events.is_empty() {
        return false;
    }

    if !INTERACTIVE_LOOP_ACTIVE.load(Ordering::Relaxed) {
        if let Err(e) = ym2151_log_play_server::client::start_interactive() {
            eprintln!("Failed to start interactive mode for loop: {}", e);
            return false;
        }
        INTERACTIVE_LOOP_ACTIVE.store(true, Ordering::Relaxed);
    }

    send_loop_json(log)
}

/// Queue the next loop iteration without restarting interactive mode.
#[cfg(all(windows, not(test)))]
pub fn queue_loop_playback(log: &Ym2151Log) -> bool {
    if log.events.is_empty() {
        return false;
    }

    if !INTERACTIVE_LOOP_ACTIVE.load(Ordering::Relaxed) {
        if let Err(e) = ym2151_log_play_server::client::start_interactive() {
            eprintln!("Failed to start interactive mode for loop: {}", e);
            return false;
        }
        INTERACTIVE_LOOP_ACTIVE.store(true, Ordering::Relaxed);
    }

    send_loop_json(log)
}

/// Stop loop playback and interactive mode.
#[cfg(all(windows, not(test)))]
pub fn stop_loop_playback() {
    if INTERACTIVE_LOOP_ACTIVE.swap(false, Ordering::Relaxed) {
        if let Err(e) = ym2151_log_play_server::client::stop_interactive() {
            eprintln!("Failed to stop interactive mode: {}", e);
        }
    }
}

#[cfg(all(windows, not(test)))]
fn send_loop_json(log: &Ym2151Log) -> bool {
    match serde_json::to_string(log) {
        Ok(json_string) => {
            if let Err(e) = ym2151_log_play_server::client::play_json_interactive(&json_string) {
                eprintln!("Loop playback error: {}", e);
                false
            } else {
                true
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize loop JSON: {}", e);
            false
        }
    }
}

#[cfg(all(windows, not(test)))]
static INTERACTIVE_LOOP_ACTIVE: AtomicBool = AtomicBool::new(false);

#[cfg(any(not(windows), test))]
pub fn start_loop_playback(log: &Ym2151Log) -> bool {
    !log.events.is_empty()
}

#[cfg(any(not(windows), test))]
pub fn queue_loop_playback(log: &Ym2151Log) -> bool {
    !log.events.is_empty()
}

#[cfg(any(not(windows), test))]
pub fn stop_loop_playback() {}
