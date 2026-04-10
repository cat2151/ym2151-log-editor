use crate::models::Ym2151Event;

#[test]
fn description_formats_keyon_for_single_digit_addr() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "8".to_string(),
        data: "78".to_string(),
    };

    assert_eq!(event.description(), "ch0 keyon");
}

#[test]
fn description_formats_keyoff_when_operator_bits_are_clear() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "03".to_string(),
    };

    assert_eq!(event.description(), "ch3 keyoff");
}

#[test]
fn description_formats_channel_register_range() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "24".to_string(),
        data: "4F".to_string(),
    };

    assert_eq!(event.description(), "ch4 pan / feedback / algorithm");
}

#[test]
fn description_formats_operator_register_range() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "48".to_string(),
        data: "16".to_string(),
    };

    assert_eq!(event.description(), "ch0 c1 detune1 / multiple");
}

#[test]
fn description_formats_prefixed_hex_values() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "0xA8".to_string(),
        data: "0x05".to_string(),
    };

    assert_eq!(event.description(), "ch0 c1 am enable / decay1 rate");
}
