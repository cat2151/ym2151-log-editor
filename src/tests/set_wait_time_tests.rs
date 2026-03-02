use crate::app::App;
use crate::models::Ym2151Event;
use crate::time_display::TimeDisplayMode;

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
    app.navigation.selected_index = 1;
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

    app.navigation.selected_index = 1;
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
    app.navigation.selected_index = 0;
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
    app.navigation.selected_index = 1;
    app.set_wait_time_ms(0);

    // Verify event 1 now has timestamp 0.0 (same as previous event)
    assert!((app.log.events[1].time - 0.0).abs() < 0.0001);

    // Verify event 2 was also adjusted (should be 0.01, was 0.02, delta = -0.01)
    assert!((app.log.events[2].time - 0.01).abs() < 0.0001);
}
