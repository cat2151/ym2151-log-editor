use crate::app::App;
use crate::models::Ym2151Event;

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
