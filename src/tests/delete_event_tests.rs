use crate::app::App;
use crate::models::Ym2151Event;

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
