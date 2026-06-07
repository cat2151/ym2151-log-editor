use crate::app::App;

const VALID_JSON: &str = r#"{
    "events": [
        { "time": 0.0, "addr": "28", "data": "00" },
        { "time": 0.01, "addr": "08", "data": "78" }
    ]
}"#;

const VALID_JSON_WITH_PREFIXED_HEX: &str = r#"{
    "events": [
        { "time": 0.0, "addr": "0x20", "data": "0xD7" },
        { "time": 0.0, "addr": "0XA8", "data": "0X05" },
        { "time": 0.25, "addr": "0x08", "data": "0x00" }
    ]
}"#;

#[test]
fn test_load_from_str_success() {
    let mut app = App::new();
    let result = app.load_from_str(VALID_JSON);
    assert!(result.is_ok());
    assert_eq!(app.log.events.len(), 2);
    assert_eq!(app.log.events[0].addr, "28");
    assert_eq!(app.log.events[1].addr, "08");
}

#[test]
fn test_load_from_str_file_path_is_none() {
    let mut app = App::new();
    app.file_path = Some("some_file.json".to_string());
    let _ = app.load_from_str(VALID_JSON);
    assert!(app.file_path.is_none());
}

#[test]
fn test_load_from_str_navigation_reset() {
    let mut app = App::new();
    // Pre-load some events and navigate away from top
    app.log.events = vec![
        crate::models::Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        crate::models::Ym2151Event {
            time: 0.01,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
    ];
    app.navigation.selected_index = 1;
    app.navigation.scroll_offset = 1;

    let _ = app.load_from_str(VALID_JSON);
    assert_eq!(app.navigation.selected_index, 0);
    assert_eq!(app.navigation.scroll_offset, 0);
}

#[test]
fn test_load_from_str_invalid_json() {
    let mut app = App::new();
    let result = app.load_from_str("not valid json {{{");
    assert!(result.is_err());
}

#[test]
fn test_load_from_str_invalid_json_preserves_current_state() {
    let mut app = App::new();
    app.log.events = vec![crate::models::Ym2151Event {
        time: 0.25,
        addr: "20".to_string(),
        data: "4F".to_string(),
    }];
    app.file_path = Some("current.json".to_string());
    app.navigation.selected_index = 1;
    app.navigation.scroll_offset = 1;

    let result = app.load_from_str("not valid json {{{");

    assert!(result.is_err());
    assert_eq!(app.log.events.len(), 1);
    assert_eq!(app.log.events[0].addr, "20");
    assert_eq!(app.file_path.as_deref(), Some("current.json"));
    assert_eq!(app.navigation.selected_index, 1);
    assert_eq!(app.navigation.scroll_offset, 1);
}

#[test]
fn test_load_from_str_wrong_schema() {
    let mut app = App::new();
    // Valid JSON but not a Ym2151Log schema
    let result = app.load_from_str(r#"{"foo": "bar"}"#);
    assert!(result.is_err());
}

#[test]
fn test_load_from_str_prefixed_hex_values_keep_description_available() {
    let mut app = App::new();

    let result = app.load_from_str(VALID_JSON_WITH_PREFIXED_HEX);

    assert!(result.is_ok());
    assert_eq!(
        app.log
            .events
            .iter()
            .map(|event| event.description())
            .collect::<Vec<_>>(),
        vec![
            "ch0 pan / feedback / algorithm",
            "ch0 c1 am enable / decay1 rate",
            "ch0 keyoff",
        ]
    );
}
