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

#[test]
fn test_delete_selected_event() {
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

    // Select middle event and delete it
    app.navigation.selected_index = 1;
    app.delete_selected_event();

    // Verify event count decreased
    assert_eq!(app.log.events.len(), 2);

    // Verify the correct event was deleted (remaining events should be index 0 and 2)
    assert_eq!(app.log.events[0].addr, "20");
    assert_eq!(app.log.events[1].addr, "60");

    // Verify selected_index is still valid
    assert_eq!(app.navigation.selected_index, 1);
}

#[test]
fn test_delete_last_event() {
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

    // Select last event and delete it
    app.navigation.selected_index = 1;
    app.delete_selected_event();

    // Verify event count decreased
    assert_eq!(app.log.events.len(), 1);

    // Verify selected_index was adjusted to last valid index
    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_delete_single_event() {
    let mut app = App::new();
    app.log.events = vec![Ym2151Event {
        time: 0.0,
        addr: "20".to_string(),
        data: "4F".to_string(),
    }];

    // Select the only event and delete it
    app.navigation.selected_index = 0;
    app.delete_selected_event();

    // Verify all events are deleted
    assert_eq!(app.log.events.len(), 0);

    // selected_index should remain 0 (though there are no events)
    assert_eq!(app.navigation.selected_index, 0);
}

#[test]
fn test_delete_empty_list() {
    let mut app = App::new();
    app.log.events = vec![];

    // Try to delete from empty list (should not panic)
    app.navigation.selected_index = 0;
    app.delete_selected_event();

    // Verify still empty
    assert_eq!(app.log.events.len(), 0);
}

#[test]
fn test_move_down_to_empty_line() {
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

    // Start at first event
    app.navigation.selected_index = 0;

    // Move down to second event
    app.move_down();
    assert_eq!(app.navigation.selected_index, 1);

    // Move down to empty line (one beyond last event)
    app.move_down();
    assert_eq!(app.navigation.selected_index, 2);
    assert_eq!(app.navigation.selected_index, app.log.events.len());

    // Try to move down again (should stay at empty line)
    app.move_down();
    assert_eq!(app.navigation.selected_index, 2);
}

#[test]
fn test_move_down_empty_log() {
    let mut app = App::new();
    app.log.events = vec![];

    // Start at index 0 (empty)
    app.navigation.selected_index = 0;

    // Try to move down (should stay at 0)
    app.move_down();
    assert_eq!(app.navigation.selected_index, 0);
}

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
