use serde::{Deserialize, Serialize};

/// JSON event structure for ym2151-log
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ym2151Event {
    pub time: f64,
    pub addr: String,
    pub data: String,
}

/// JSON log structure for ym2151-log
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ym2151Log {
    pub events: Vec<Ym2151Event>,
}

impl Ym2151Event {
    /// Check if this event is a KeyON event (data value 0x00 to 0xFF on register 0x08)
    pub fn is_key_on(&self) -> bool {
        self.addr.to_uppercase() == "08"
    }
}
