use crate::app::App;
use crate::models::Ym2151Event;

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
