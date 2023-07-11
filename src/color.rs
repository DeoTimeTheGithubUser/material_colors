use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Color {
    hex: u32,
}

impl Color {
    pub const fn from_hex(hex: u32) -> Color {
        Color { hex }
    }

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self::from_hex(((r as u32) << 16) + ((g as u32) << 8) + (b as u32))
    }

    pub const fn r(&self) -> u8 {
        ((self.hex << 16) & 0xFF) as u8
    }

    pub const fn g(&self) -> u8 {
        ((self.hex << 8) & 0xFF) as u8
    }

    pub const fn b(&self) -> u8 {
        (self.hex & 0xFF) as u8
    }

    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::from_hex(0xFFFFFFFF)
    }
}