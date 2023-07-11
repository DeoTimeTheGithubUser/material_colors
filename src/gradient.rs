use std::ops::Index;
use serde::{Deserialize, Serialize};
use crate::color::Color;
use crate::hue::Black;

#[derive(Serialize, Deserialize, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Gradient {
    pub stops: Vec<Color>
}

impl Gradient {
    pub const fn new(stops: Vec<Color>) -> Self {
        Gradient { stops }
    }



}

impl<const N: usize> From<[Color; N]> for Gradient {
    fn from(value: [Color; N]) -> Self {
        Self::new(value.to_vec())
    }
}

#[macro_export]
macro_rules! gradient {
    ($($color:expr),*) => {{
        let v = vec![];
        $(
        v.push($color.into());
        )*
        $crate::gradient::Gradient::new(v)
    }};
}
