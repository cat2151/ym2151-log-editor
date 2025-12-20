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
    /// Check if this event is a KeyON event (register 0x08)
    pub fn is_key_on(&self) -> bool {
        self.addr.to_uppercase() == "08"
    }

    /// Check if this event is a KEYOFF event (register 0x08 with bits 3,4,5,6 all zero)
    /// In YM2151, register 0x08 controls KEY ON/OFF. Bits 3-6 specify operators.
    /// If all of bits 3,4,5,6 are 0, it's a KEY OFF operation.
    pub fn is_key_off(&self) -> bool {
        if self.addr.to_uppercase() != "08" {
            return false;
        }

        // Parse hex data value
        if let Ok(value) = u8::from_str_radix(&self.data, 16) {
            // Check if bits 3,4,5,6 are all 0
            // Bit mask for bits 3-6: 0b01111000 = 0x78
            (value & 0x78) == 0
        } else {
            false
        }
    }
}
