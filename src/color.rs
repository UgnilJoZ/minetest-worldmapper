use image::Rgba;
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Rgba<u8>);

impl Default for Color {
    fn default() -> Self {
        Color(Rgba([0,0,0,0]))
    }
}

impl Color {
    pub fn alpha(&self) -> u8 {
        self.0[3]
    }

    pub fn with_background(&self, other: &Color) -> Color {
        if self.alpha() == 255 {
            return *self;
        } else if self.alpha() == 0 {
            return *other;
        }
        let fore_alpha = self.alpha() as f32 / 255.0;
        let back_alpha = 1.0 - fore_alpha;
        let r = fore_alpha * (self.0[0] as f32) + back_alpha * (other.0[0] as f32);
        let g = fore_alpha * (self.0[1] as f32) + back_alpha * (other.0[1] as f32);
        let b = fore_alpha * (self.0[2] as f32) + back_alpha * (other.0[2] as f32);
        let a = fore_alpha + back_alpha * (other.alpha() as f32 / 255.0);
        Color(Rgba::from([r as u8, g as u8, b as u8, (255.0 * a) as u8]))
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a hexadecimal color string in the format 'abcdef' (rgb) or 'abcdef01' (rgba)"
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let valid = ((s.len() == 6) || (s.len() == 8)) && s.chars().all(|c| c.is_digit(16));
        if valid {
            // We ruled out a ParseIntError above, so can call unwrap
            let r = u8::from_str_radix(&s[0..2], 16).unwrap();
            let g = u8::from_str_radix(&s[2..4], 16).unwrap();
            let b = u8::from_str_radix(&s[4..6], 16).unwrap();
            let alpha: String = s.chars().skip(6).take(2).collect();
            // An error here can only mean there is no alpha provided, so we can discard the error
            let a = u8::from_str_radix(&alpha, 16).unwrap_or(255);
            Ok(Color(Rgba::from([r, g, b, a])))
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}
