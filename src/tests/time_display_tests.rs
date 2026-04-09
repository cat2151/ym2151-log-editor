use crate::models::{Ym2151Event, Ym2151Log};
use crate::time_display::{format_event, TimeDisplayMode};

#[test]
fn format_event_appends_keyon_description() {
    let log = Ym2151Log {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "08".to_string(),
            data: "78".to_string(),
        }],
    };

    assert_eq!(
        format_event(&log, 0, TimeDisplayMode::Cumulative),
        "0.000000  08  78  ch0 keyon"
    );
}

#[test]
fn format_event_appends_operator_description() {
    let log = Ym2151Log {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "60".to_string(),
            data: "14".to_string(),
        }],
    };

    assert_eq!(
        format_event(&log, 0, TimeDisplayMode::Cumulative),
        "0.000000  60  14  ch0 m1 tl"
    );
}
