#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Color {
    hex: u32,
}

impl Color {
    pub const fn from_hex(hex: u32) -> Color {
        Color { hex }
    }
}
