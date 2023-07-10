use crate::{S500, Shade};

pub enum Hue<const S: Shade = S500> {
    Black,
    White,
    Red,
    Pink,
    Purple,
    DeepPurple,
    Indigo,
    Blue,
    LightBlue,
    Cyan,
    Teal,
    Green,
    LightGreen,
    Lime,
    Yellow,
    Amber,
    Orange,
    DeepOrange,
    Brown,
    Gray,
    BlueGray,
}

include!(concat!(env!("OUT_DIR"), "/hue_impls.rs"));