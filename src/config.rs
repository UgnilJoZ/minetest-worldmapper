use crate::color::Color;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub version: u16,
    pub background_color: Color,
    /// Which opacity is considered enough to
    /// continue with the next pixel
    #[serde(default = "default_sufficient_alpha")]
    pub sufficient_alpha: u8,
    pub node_colors: HashMap<String, Color>,
}

const fn default_sufficient_alpha() -> u8 {
    230
}
