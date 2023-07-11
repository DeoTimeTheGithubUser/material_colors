use crate::color::Color;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gradient {
    pub stops: Vec<Color>,
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
        $crate::gradient::Gradient::from([$($color.into()),*])
    }};
}

#[cfg(feature = "colorgrad")]
impl From<Gradient> for colorgrad::Gradient {
    fn from(value: Gradient) -> Self {
        let colors: Vec<colorgrad::Color> =
            value.stops.iter().map(colorgrad::Color::from).collect();

        colorgrad::CustomGradient::new()
            .colors(colors.as_slice())
            .build()
            .expect("Failed to build gradient.") // should not ever fail
    }
}
