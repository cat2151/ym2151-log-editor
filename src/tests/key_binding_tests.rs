use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
    assert!(crate::is_fast_move_up_key(
        &KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE),
        false
    ));
    assert!(crate::is_fast_move_down_key(
        &KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE),
        false
    ));
}

#[test]
fn ctrl_u_and_ctrl_d_trigger_ten_line_navigation() {
    assert!(crate::is_fast_move_up_key(
        &KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL),
        false
    ));
    assert!(crate::is_fast_move_down_key(
        &KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL),
        false
    ));
}

#[test]
fn nine_prefix_enables_vim_style_ten_line_navigation() {
    assert!(crate::is_fast_move_up_key(
        &KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        true
    ));
    assert!(crate::is_fast_move_down_key(
        &KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        true
    ));
}
