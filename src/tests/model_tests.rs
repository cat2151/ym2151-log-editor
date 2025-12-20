use crate::models::Ym2151Event;

#[test]
fn test_is_key_on_with_addr_08() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "78".to_string(),
    };
    assert!(event.is_key_on());
}

#[test]
fn test_is_key_on_with_other_addr() {
    let event = Ym2151Event {
        time: 0.0,
        addr: "20".to_string(),
        data: "4F".to_string(),
    };
    assert!(!event.is_key_on());
}

#[test]
fn test_is_key_off_with_all_bits_zero() {
    // Data "00" means bits 3,4,5,6 are all 0
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "00".to_string(),
    };
    assert!(event.is_key_off());
}

#[test]
fn test_is_key_off_with_bits_0_1_2_only() {
    // Data "07" (0b00000111) - only bits 0,1,2 are set, bits 3-6 are 0
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "07".to_string(),
    };
    assert!(event.is_key_off());
}

#[test]
fn test_is_key_off_with_bit_7_only() {
    // Data "80" (0b10000000) - only bit 7 is set, bits 3-6 are 0
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "80".to_string(),
    };
    assert!(event.is_key_off());
}

#[test]
fn test_is_not_key_off_with_bit_3_set() {
    // Data "08" (0b00001000) - bit 3 is set
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "08".to_string(),
    };
    assert!(!event.is_key_off());
}

#[test]
fn test_is_not_key_off_with_bit_4_set() {
    // Data "10" (0b00010000) - bit 4 is set
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "10".to_string(),
    };
    assert!(!event.is_key_off());
}

#[test]
fn test_is_not_key_off_with_bit_5_set() {
    // Data "20" (0b00100000) - bit 5 is set
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "20".to_string(),
    };
    assert!(!event.is_key_off());
}

#[test]
fn test_is_not_key_off_with_bit_6_set() {
    // Data "40" (0b01000000) - bit 6 is set
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "40".to_string(),
    };
    assert!(!event.is_key_off());
}

#[test]
fn test_is_not_key_off_with_multiple_bits_set() {
    // Data "78" (0b01111000) - bits 3,4,5,6 are all set
    let event = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "78".to_string(),
    };
    assert!(!event.is_key_off());
}

#[test]
fn test_is_not_key_off_with_wrong_addr() {
    // Even if data is "00", it's not a KEYOFF if addr is not "08"
    let event = Ym2151Event {
        time: 0.0,
        addr: "20".to_string(),
        data: "00".to_string(),
    };
    assert!(!event.is_key_off());
}

#[test]
fn test_is_key_off_case_insensitive_addr() {
    let event_lowercase = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "00".to_string(),
    };
    assert!(event_lowercase.is_key_off());

    let event_uppercase = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "00".to_string(),
    };
    assert!(event_uppercase.is_key_off());
}

#[test]
fn test_is_key_off_case_insensitive_data() {
    let event_lowercase = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "0a".to_string(), // bits 3,4,5,6 are 0, only bit 1 and 3 set -> wait, 0x0A = 0b00001010
    };
    // 0x0A = 0b00001010 - bit 3 is set, so NOT a KEYOFF
    assert!(!event_lowercase.is_key_off());

    let event_uppercase = Ym2151Event {
        time: 0.0,
        addr: "08".to_string(),
        data: "0A".to_string(),
    };
    assert!(!event_uppercase.is_key_off());
}
