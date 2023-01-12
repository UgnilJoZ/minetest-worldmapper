use crate::color::Color;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct HillShading {
    #[serde(default)]
    pub on: bool,
    #[serde(default = "default_surface_density")]
    pub surface_density: u8,
}

impl Default for HillShading {
    fn default() -> Self {
        HillShading {
            on: false,
            surface_density: default_surface_density(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub version: u16,
    pub background_color: Color,
    /// Which opacity is considered enough to
    /// continue with the next pixel
    #[serde(default = "default_sufficient_alpha")]
    pub sufficient_alpha: u8,
    pub node_colors: HashMap<String, Color>,
    #[serde(default)]
    pub hill_shading: HillShading,
}

const fn default_sufficient_alpha() -> u8 {
    230
}

const fn default_surface_density() -> u8 {
    128
}

