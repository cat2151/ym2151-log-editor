use crate::models::Ym2151Log;

#[cfg(all(windows, not(test)))]
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(any(all(windows, not(test)), test))]
trait InteractivePlaybackClient {
    fn start_interactive(&self) -> Result<(), String>;
    fn play_json_interactive(&self, json: &str) -> Result<(), String>;
    fn stop_interactive(&self) -> Result<(), String>;
}

#[cfg(any(all(windows, not(test)), test))]
fn start_loop_playback_with_client(
    log: &Ym2151Log,
    interactive_loop_active: &mut bool,
    client: &impl InteractivePlaybackClient,
) -> bool {
    if log.events.is_empty() {
        return false;
    }

    if !ensure_interactive_started(interactive_loop_active, client) {
        return false;
    }

    send_loop_json_with_client(log, client)
}

#[cfg(any(all(windows, not(test)), test))]
fn queue_loop_playback_with_client(
    log: &Ym2151Log,
    interactive_loop_active: &mut bool,
    client: &impl InteractivePlaybackClient,
) -> bool {
    if log.events.is_empty() {
        return false;
    }

    if !ensure_interactive_started(interactive_loop_active, client) {
        return false;
    }

    send_loop_json_with_client(log, client)
}

#[cfg(any(all(windows, not(test)), test))]
fn stop_loop_playback_with_client(
    interactive_loop_active: &mut bool,
    client: &impl InteractivePlaybackClient,
) {
    if *interactive_loop_active {
        *interactive_loop_active = false;
        if let Err(e) = client.stop_interactive() {
            eprintln!("Failed to stop interactive mode: {}", e);
        }
    }
}

#[cfg(any(all(windows, not(test)), test))]
fn ensure_interactive_started(
    interactive_loop_active: &mut bool,
    client: &impl InteractivePlaybackClient,
) -> bool {
    if !*interactive_loop_active {
        if let Err(e) = client.start_interactive() {
            eprintln!("Failed to start interactive mode for loop: {}", e);
            return false;
        }
        *interactive_loop_active = true;
    }

    true
}

#[cfg(any(all(windows, not(test)), test))]
fn send_loop_json_with_client(log: &Ym2151Log, client: &impl InteractivePlaybackClient) -> bool {
    match serde_json::to_string(log) {
        Ok(json_string) => {
            if let Err(e) = client.play_json_interactive(&json_string) {
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
struct Ym2151LogPlayServerClient;

#[cfg(all(windows, not(test)))]
impl InteractivePlaybackClient for Ym2151LogPlayServerClient {
    fn start_interactive(&self) -> Result<(), String> {
        ym2151_log_play_server::client::start_interactive().map_err(|e| e.to_string())
    }

    fn play_json_interactive(&self, json: &str) -> Result<(), String> {
        ym2151_log_play_server::client::play_json_interactive(json).map_err(|e| e.to_string())
    }

    fn stop_interactive(&self) -> Result<(), String> {
        ym2151_log_play_server::client::stop_interactive().map_err(|e| e.to_string())
    }
}

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
    let mut interactive_loop_active = INTERACTIVE_LOOP_ACTIVE.load(Ordering::Relaxed);
    let started = start_loop_playback_with_client(
        log,
        &mut interactive_loop_active,
        &Ym2151LogPlayServerClient,
    );
    INTERACTIVE_LOOP_ACTIVE.store(interactive_loop_active, Ordering::Relaxed);
    started
}

/// Queue the next loop iteration without restarting interactive mode.
#[cfg(all(windows, not(test)))]
pub fn queue_loop_playback(log: &Ym2151Log) -> bool {
    let mut interactive_loop_active = INTERACTIVE_LOOP_ACTIVE.load(Ordering::Relaxed);
    let queued = queue_loop_playback_with_client(
        log,
        &mut interactive_loop_active,
        &Ym2151LogPlayServerClient,
    );
    INTERACTIVE_LOOP_ACTIVE.store(interactive_loop_active, Ordering::Relaxed);
    queued
}

/// Stop loop playback and interactive mode.
#[cfg(all(windows, not(test)))]
pub fn stop_loop_playback() {
    let mut interactive_loop_active = INTERACTIVE_LOOP_ACTIVE.load(Ordering::Relaxed);
    stop_loop_playback_with_client(&mut interactive_loop_active, &Ym2151LogPlayServerClient);
    INTERACTIVE_LOOP_ACTIVE.store(interactive_loop_active, Ordering::Relaxed);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Ym2151Event;
    use std::cell::RefCell;

    #[derive(Debug, PartialEq, Eq)]
    enum MockCall {
        StartInteractive,
        PlayJson(String),
        StopInteractive,
    }

    #[derive(Default)]
    struct MockInteractivePlaybackClient {
        calls: RefCell<Vec<MockCall>>,
        fail_start: bool,
    }

    impl InteractivePlaybackClient for MockInteractivePlaybackClient {
        fn start_interactive(&self) -> Result<(), String> {
            self.calls.borrow_mut().push(MockCall::StartInteractive);
            if self.fail_start {
                Err("failed to start".to_string())
            } else {
                Ok(())
            }
        }

        fn play_json_interactive(&self, json: &str) -> Result<(), String> {
            self.calls
                .borrow_mut()
                .push(MockCall::PlayJson(json.to_string()));
            Ok(())
        }

        fn stop_interactive(&self) -> Result<(), String> {
            self.calls.borrow_mut().push(MockCall::StopInteractive);
            Ok(())
        }
    }

    fn sample_log(time: f64) -> Ym2151Log {
        Ym2151Log {
            events: vec![Ym2151Event {
                time,
                addr: "20".to_string(),
                data: "4F".to_string(),
            }],
        }
    }

    #[test]
    fn start_loop_playback_starts_interactive_and_sends_json() {
        let log = sample_log(0.05);
        let mut interactive_loop_active = false;
        let client = MockInteractivePlaybackClient::default();

        let started =
            start_loop_playback_with_client(&log, &mut interactive_loop_active, &client);

        assert!(started);
        assert!(interactive_loop_active);
        assert_eq!(
            *client.calls.borrow(),
            vec![
                MockCall::StartInteractive,
                MockCall::PlayJson(serde_json::to_string(&log).unwrap())
            ]
        );
    }

    #[test]
    fn queue_loop_playback_reuses_existing_interactive_session() {
        let first_log = sample_log(0.05);
        let second_log = sample_log(0.10);
        let mut interactive_loop_active = false;
        let client = MockInteractivePlaybackClient::default();

        assert!(start_loop_playback_with_client(
            &first_log,
            &mut interactive_loop_active,
            &client
        ));
        assert!(queue_loop_playback_with_client(
            &second_log,
            &mut interactive_loop_active,
            &client
        ));

        assert!(interactive_loop_active);
        assert_eq!(
            *client.calls.borrow(),
            vec![
                MockCall::StartInteractive,
                MockCall::PlayJson(serde_json::to_string(&first_log).unwrap()),
                MockCall::PlayJson(serde_json::to_string(&second_log).unwrap())
            ]
        );
    }

    #[test]
    fn start_loop_playback_returns_false_when_interactive_start_fails() {
        let log = sample_log(0.05);
        let mut interactive_loop_active = false;
        let client = MockInteractivePlaybackClient {
            fail_start: true,
            ..Default::default()
        };

        let started =
            start_loop_playback_with_client(&log, &mut interactive_loop_active, &client);

        assert!(!started);
        assert!(!interactive_loop_active);
        assert_eq!(
            *client.calls.borrow(),
            vec![MockCall::StartInteractive]
        );
    }

    #[test]
    fn stop_loop_playback_stops_interactive_mode_once() {
        let mut interactive_loop_active = true;
        let client = MockInteractivePlaybackClient::default();

        stop_loop_playback_with_client(&mut interactive_loop_active, &client);
        stop_loop_playback_with_client(&mut interactive_loop_active, &client);

        assert!(!interactive_loop_active);
        assert_eq!(*client.calls.borrow(), vec![MockCall::StopInteractive]);
    }
}
