use crate::app::App;
use crate::models::Ym2151Event;

const VALID_PASTE_JSON: &str = r#"{
    "events": [
        { "time": 0.1, "addr": "28", "data": "00" },
        { "time": 0.12, "addr": "08", "data": "78" }
    ]
}"#;

const LONG_PASTE_JSON: &str = r#"{
    "events": [
        { "time": 0.1, "addr": "28", "data": "00" },
        { "time": 0.6, "addr": "08", "data": "78" }
    ]
}"#;

#[test]
fn test_insert_event_before_selected_at_start() {
    let mut app = App::new();
    app.log.events = vec![
        Ym2151Event {
            time: 0.01,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        Ym2151Event {
            time: 0.02,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
    ];

    // Insert before first event
    app.navigation.selected_index = 0;
    app.insert_event_before_selected();

    // Verify event count increased
    assert_eq!(app.log.events.len(), 3);

    // Verify new event inserted at position 0
    assert_eq!(app.log.events[0].addr, "00");
    assert_eq!(app.log.events[0].data, "00");
    assert!((app.log.events[0].time - 0.0).abs() < 0.0001);

    // Verify original events shifted
    assert_eq!(app.log.events[1].addr, "20");
    assert_eq!(app.log.events[2].addr, "40");

    // Verify selected_index stayed on the new event
    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_insert_event_before_selected_in_middle() {
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

    // Insert before middle event (index 1)
    app.navigation.selected_index = 1;
    app.insert_event_before_selected();

    // Verify event count increased
    assert_eq!(app.log.events.len(), 4);

    // Verify new event inserted at position 1 with time from previous event
    assert_eq!(app.log.events[1].addr, "00");
    assert_eq!(app.log.events[1].data, "00");
    assert!((app.log.events[1].time - 0.0).abs() < 0.0001);

    // Verify original events
    assert_eq!(app.log.events[0].addr, "20");
    assert_eq!(app.log.events[2].addr, "40");
    assert_eq!(app.log.events[3].addr, "60");

    // Verify selected_index stayed on the new event
    assert_eq!(app.navigation.selected_index, 1);
}

#[test]
fn test_insert_event_before_selected_at_end() {
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

    // Move cursor to empty line after last event
    app.navigation.selected_index = 2;
    app.insert_event_before_selected();

    // Verify event count increased
    assert_eq!(app.log.events.len(), 3);

    // Verify new event inserted at position 2 with time from last event
    assert_eq!(app.log.events[2].addr, "00");
    assert_eq!(app.log.events[2].data, "00");
    assert!((app.log.events[2].time - 0.01).abs() < 0.0001);

    // Verify original events unchanged
    assert_eq!(app.log.events[0].addr, "20");
    assert_eq!(app.log.events[1].addr, "40");

    // Verify selected_index stayed at 2 (now pointing to the new event)
    assert_eq!(app.navigation.selected_index, 2);
}

#[test]
fn test_insert_event_before_selected_empty_list() {
    let mut app = App::new();
    app.log.events = vec![];

    // Insert into empty list
    app.navigation.selected_index = 0;
    app.insert_event_before_selected();

    // Verify event count increased
    assert_eq!(app.log.events.len(), 1);

    // Verify new event created with time 0.0
    assert_eq!(app.log.events[0].addr, "00");
    assert_eq!(app.log.events[0].data, "00");
    assert!((app.log.events[0].time - 0.0).abs() < 0.0001);

    // Verify selected_index is still 0
    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_insert_event_scroll_adjustment() {
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

    // Set scroll_offset ahead of selected_index
    app.navigation.selected_index = 0;
    app.navigation.scroll_offset = 1;

    app.insert_event_before_selected();

    // Verify scroll_offset was adjusted to keep new event visible
    assert_eq!(app.navigation.scroll_offset, 0);
}

#[test]
fn test_insert_events_from_str_before_selected_in_middle() {
    let mut app = App::new();
    app.file_path = Some("test_data/sample.json".to_string());
    app.log.events = vec![
        Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        Ym2151Event {
            time: 0.05,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
        Ym2151Event {
            time: 0.10,
            addr: "60".to_string(),
            data: "14".to_string(),
        },
    ];
    app.navigation.selected_index = 1;

    let result = app.insert_events_from_str_before_selected(VALID_PASTE_JSON);

    assert!(result.is_ok());
    assert_eq!(app.file_path.as_deref(), Some("test_data/sample.json"));
    assert_eq!(app.log.events.len(), 5);
    assert_eq!(app.navigation.selected_index, 1);
    assert_eq!(app.log.events[0].addr, "20");
    assert_eq!(app.log.events[1].addr, "28");
    assert_eq!(app.log.events[1].data, "00");
    assert!((app.log.events[1].time - 0.0).abs() < 0.0001);
    assert_eq!(app.log.events[2].addr, "08");
    assert_eq!(app.log.events[2].data, "78");
    assert!((app.log.events[2].time - 0.02).abs() < 0.0001);
    assert_eq!(app.log.events[3].addr, "40");
    assert!((app.log.events[3].time - 0.05).abs() < 0.0001);
}

#[test]
fn test_insert_events_from_str_before_selected_at_end() {
    let mut app = App::new();
    app.log.events = vec![
        Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        Ym2151Event {
            time: 0.05,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
    ];
    app.navigation.selected_index = 2;

    let result = app.insert_events_from_str_before_selected(VALID_PASTE_JSON);

    assert!(result.is_ok());
    assert_eq!(app.log.events.len(), 4);
    assert!((app.log.events[2].time - 0.05).abs() < 0.0001);
    assert!((app.log.events[3].time - 0.07).abs() < 0.0001);
}

#[test]
fn test_insert_events_from_str_before_selected_invalid_json_keeps_log_unchanged() {
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "20".to_string(),
        data: "4F".to_string(),
    }];

    let result = app.insert_events_from_str_before_selected("not valid json {{{");

    assert!(result.is_err());
    assert_eq!(app.log.events.len(), 1);
    assert_eq!(app.log.events[0].addr, "20");
    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_insert_events_from_str_before_selected_shifts_following_events_when_needed() {
    let mut app = App::new();
    app.log.events = vec![
        Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        Ym2151Event {
            time: 0.05,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
        Ym2151Event {
            time: 0.06,
            addr: "60".to_string(),
            data: "14".to_string(),
        },
    ];
    app.navigation.selected_index = 1;

    let result = app.insert_events_from_str_before_selected(LONG_PASTE_JSON);

    assert!(result.is_ok());
    assert_eq!(app.log.events.len(), 5);
    assert!((app.log.events[1].time - 0.0).abs() < 0.0001);
    assert!((app.log.events[2].time - 0.5).abs() < 0.0001);
    assert!((app.log.events[3].time - 0.5).abs() < 0.0001);
    assert!((app.log.events[4].time - 0.51).abs() < 0.0001);
}
