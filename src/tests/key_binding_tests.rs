use crate::{app::App, models::Ym2151Event, PendingCountPrefix, COUNT_PREFIX_TIMEOUT};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

#[test]
fn vim_style_navigation_keys_are_supported() {
    assert!(crate::is_move_up_key(&KeyCode::Char('k')));
    assert!(crate::is_move_down_key(&KeyCode::Char('j')));
}

#[test]
fn uppercase_vim_style_navigation_keys_are_supported() {
    assert!(crate::is_move_up_key(&KeyCode::Char('K')));
    assert!(crate::is_move_down_key(&KeyCode::Char('J')));
}

#[test]
fn arrow_keys_remain_supported_navigation_keys() {
    assert!(crate::is_move_up_key(&KeyCode::Up));
    assert!(crate::is_move_down_key(&KeyCode::Down));
}

#[test]
fn unrelated_keys_do_not_trigger_navigation() {
    assert!(!crate::is_move_up_key(&KeyCode::Char('j')));
    assert!(!crate::is_move_down_key(&KeyCode::Char('k')));
}

#[test]
fn page_keys_trigger_ten_line_navigation() {
    assert!(crate::is_fast_move_up_key(&KeyEvent::new(
        KeyCode::PageUp,
        KeyModifiers::NONE
    )));
    assert!(crate::is_fast_move_down_key(&KeyEvent::new(
        KeyCode::PageDown,
        KeyModifiers::NONE
    )));
}

#[test]
fn ctrl_u_and_ctrl_d_trigger_ten_line_navigation() {
    assert!(crate::is_fast_move_up_key(&KeyEvent::new(
        KeyCode::Char('u'),
        KeyModifiers::CONTROL
    )));
    assert!(crate::is_fast_move_down_key(&KeyEvent::new(
        KeyCode::Char('d'),
        KeyModifiers::CONTROL
    )));
}

#[test]
fn count_prefix_is_not_treated_as_ten_line_shortcut() {
    assert!(!crate::is_fast_move_up_key(&KeyEvent::new(
        KeyCode::Char('k'),
        KeyModifiers::NONE
    )));
    assert!(!crate::is_fast_move_down_key(&KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE
    )));
}

#[test]
fn count_prefix_only_consumes_j_and_k() {
    assert!(crate::is_count_prefix_move_up_key(&KeyCode::Char('k')));
    assert!(crate::is_count_prefix_move_down_key(&KeyCode::Char('j')));
    assert!(!crate::is_count_prefix_move_up_key(&KeyCode::PageUp));
    assert!(!crate::is_count_prefix_move_down_key(&KeyCode::PageDown));
}

#[test]
fn count_prefix_accumulates_multiple_digits() {
    let mut prefix = crate::start_count_prefix(1, true);
    crate::append_count_prefix_digit(&mut prefix, 1);

    assert_eq!(prefix.count, 11);
    assert_eq!(crate::FAST_MOVE_AMOUNT, 10);
    assert_eq!(prefix.wait_fallback_ms, Some(1));
}

#[test]
fn count_prefix_allows_zero_after_first_digit() {
    let mut prefix = crate::start_count_prefix(1, true);
    crate::append_count_prefix_digit(&mut prefix, 0);

    assert_eq!(prefix.count, 10);
    assert_eq!(prefix.wait_fallback_ms, Some(0));
}

#[test]
fn timed_out_count_prefix_clears_without_applying_wait_in_timestamp_mode() {
    let mut app = App::new();
    app.toggle_time_mode();
    app.log.events = vec![
        Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        Ym2151Event {
            time: 0.02,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
    ];
    app.navigation.selected_index = 1;

    let mut pending = Some(PendingCountPrefix {
        started_at: Instant::now() - COUNT_PREFIX_TIMEOUT - Duration::from_millis(1),
        count: 11,
        wait_fallback_ms: None,
    });

    crate::flush_pending_count_prefix_if_timed_out(&mut app, &mut pending);

    assert!(pending.is_none());
    assert!((app.log.events[1].time - 0.02).abs() < f64::EPSILON);
}

#[test]
fn timed_out_count_prefix_applies_last_digit_wait_in_cumulative_mode() {
    let mut app = App::new();
    app.log.events = vec![
        Ym2151Event {
            time: 0.0,
            addr: "20".to_string(),
            data: "4F".to_string(),
        },
        Ym2151Event {
            time: 0.02,
            addr: "40".to_string(),
            data: "16".to_string(),
        },
    ];
    app.navigation.selected_index = 1;

    let mut pending = Some(PendingCountPrefix {
        started_at: Instant::now() - COUNT_PREFIX_TIMEOUT - Duration::from_millis(1),
        count: 12,
        wait_fallback_ms: Some(2),
    });

    crate::flush_pending_count_prefix_if_timed_out(&mut app, &mut pending);

    assert!(pending.is_none());
    assert!((app.log.events[1].time - 0.002).abs() < f64::EPSILON);
}

#[test]
fn digit_key_parsing_extracts_numeric_prefix_digits() {
    assert_eq!(crate::keycode_digit(&KeyCode::Char('0')), Some(0));
    assert_eq!(crate::keycode_digit(&KeyCode::Char('9')), Some(9));
    assert_eq!(crate::keycode_digit(&KeyCode::Char('j')), None);
}
