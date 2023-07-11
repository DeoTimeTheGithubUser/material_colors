#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub hex: u32,
}

impl Color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self {
            hex: ((r as u32) << 16) + ((g as u32) << 8) + (b as u32),
        }
    }

    pub const fn r(&self) -> u8 {
        ((self.hex >> 16) & 0xFF) as u8
    }

    pub const fn g(&self) -> u8 {
        ((self.hex >> 8) & 0xFF) as u8
    }

    pub const fn b(&self) -> u8 {
        (self.hex & 0xFF) as u8
    }

    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }
}

#[cfg(feature = "colorgrad")]
impl From<&Color> for colorgrad::Color {
    fn from(value: &Color) -> Self {
        colorgrad::Color::from(value.rgb())
    }
}