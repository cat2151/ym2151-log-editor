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

    /// Build a human-readable description for the event.
    pub fn description(&self) -> String {
        let Some(addr) = self.parse_hex_byte(&self.addr) else {
            return String::new();
        };
        let data = self.parse_hex_byte(&self.data);

        match addr {
            0x01 => String::from("test/lfo reset"),
            0x08 => self.key_on_description(data),
            0x0F => self.noise_description(data),
            0x10 => String::from("timer A high"),
            0x11 => String::from("timer A low"),
            0x12 => String::from("timer B"),
            0x14 => String::from("timer / irq control"),
            0x18 => String::from("lfo frequency"),
            0x19 => self.lfo_depth_description(data),
            0x1B => String::from("ct / lfo waveform"),
            0x20..=0x27 => format!("ch{} pan / feedback / algorithm", addr & 0x07),
            0x28..=0x2F => format!("ch{} key code", addr & 0x07),
            0x30..=0x37 => format!("ch{} key fraction", addr & 0x07),
            0x38..=0x3F => format!("ch{} pms / ams", addr & 0x07),
            0x40..=0x5F => self.operator_description(addr, "dt1 / mul"),
            0x60..=0x7F => self.operator_description(addr, "tl"),
            0x80..=0x9F => self.operator_description(addr, "ks / ar"),
            0xA0..=0xBF => self.operator_description(addr, "ame / d1r"),
            0xC0..=0xDF => self.operator_description(addr, "dt2 / d2r"),
            0xE0..=0xFF => self.operator_description(addr, "d1l / rr"),
            _ => format!("reg {:02X}", addr),
        }
    }

    fn parse_hex_byte(&self, value: &str) -> Option<u8> {
        u8::from_str_radix(value, 16).ok()
    }

    fn key_on_description(&self, data: Option<u8>) -> String {
        let Some(data) = data else {
            return String::from("key on/off");
        };
        let channel = data & 0x07;
        if (data & 0x78) == 0 {
            format!("ch{} keyoff", channel)
        } else {
            format!("ch{} keyon", channel)
        }
    }

    fn noise_description(&self, data: Option<u8>) -> String {
        let Some(data) = data else {
            return String::from("noise");
        };
        let enabled = if (data & 0x80) != 0 { "on" } else { "off" };
        format!("noise {} freq {}", enabled, data & 0x1F)
    }

    fn lfo_depth_description(&self, data: Option<u8>) -> String {
        let Some(data) = data else {
            return String::from("lfo depth");
        };
        if (data & 0x80) != 0 {
            String::from("pmd")
        } else {
            String::from("amd")
        }
    }

    fn operator_description(&self, addr: u8, parameter: &str) -> String {
        let channel = addr & 0x07;
        let operator = match (addr >> 3) & 0x03 {
            0 => "m1",
            1 => "c1",
            2 => "m2",
            _ => "c2",
        };
        format!("ch{} {} {}", channel, operator, parameter)
    }
}
