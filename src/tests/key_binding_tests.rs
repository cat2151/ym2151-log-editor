use crossterm::event::KeyCode;

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
