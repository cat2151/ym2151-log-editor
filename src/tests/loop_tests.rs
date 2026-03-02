use crate::app::App;
use crate::models::Ym2151Event;

#[test]
fn test_toggle_loop_requires_events() {
    let mut app = App::new();

    app.toggle_loop();

    assert!(!app.loop_enabled);
}

#[test]
fn test_toggle_loop_on_and_off() {
    let mut app = App::new();
    app.log.events.push(Ym2151Event {
        time: 0.01,
        addr: "20".to_string(),
        data: "4F".to_string(),
    });

    app.toggle_loop();
    assert!(app.loop_enabled);

    app.toggle_loop();
    assert!(!app.loop_enabled);
}

#[test]
fn test_toggle_loop_ignores_zero_duration() {
    let mut app = App::new();
    app.log.events.push(Ym2151Event {
        time: 0.0,
        addr: "20".to_string(),
        data: "4F".to_string(),
    });

    app.toggle_loop();

    assert!(!app.loop_enabled);
}

#[test]
fn test_tick_restarts_after_duration_elapsed() {
    let mut app = App::new();
    app.log.events.push(Ym2151Event {
        time: 0.05,
        addr: "20".to_string(),
        data: "4F".to_string(),
    });

    app.toggle_loop();
    assert!(app.loop_enabled);

    let previous_start = app.test_loop_started_at().unwrap();
    let past = previous_start - std::time::Duration::from_millis(100);
    app.test_set_loop_started_at(past);

    app.tick();

    let new_start = app.test_loop_started_at().unwrap();
    assert!(new_start > past);
}

#[test]
fn test_tick_reschedules_when_dirty() {
    let mut app = App::new();
    app.log.events.push(Ym2151Event {
        time: 0.05,
        addr: "20".to_string(),
        data: "4F".to_string(),
    });

    app.toggle_loop();
    let previous_start = app.test_loop_started_at().unwrap();
    app.test_set_loop_dirty(true);
    app.tick();

    assert!(!app.test_loop_dirty());
    let new_start = app.test_loop_started_at().unwrap();
    assert!(new_start >= previous_start);
}

#[test]
fn test_tick_disables_when_log_becomes_empty() {
    let mut app = App::new();
    app.log.events.push(Ym2151Event {
        time: 0.05,
        addr: "20".to_string(),
        data: "4F".to_string(),
    });

    app.toggle_loop();
    assert!(app.loop_enabled);

    app.log.events.clear();
    app.tick();

    assert!(!app.loop_enabled);
}
