use crate::color::Color;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct HillShading {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_terrain_min_alpha")]
    pub min_alpha: u8,
}

impl Default for HillShading {
    fn default() -> Self {
        HillShading {
            enabled: false,
            min_alpha: default_terrain_min_alpha(),
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

const fn default_terrain_min_alpha() -> u8 {
    128
}

impl Config {
    pub fn get_color(&self, itemstring: &[u8]) -> Option<&Color> {
        String::from_utf8(itemstring.to_vec())
            .ok()
            .and_then(|key| self.node_colors.get(&key))
    }
}
