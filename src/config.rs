use crate::color::Color;
use serde_derive::Deserialize;
#[cfg(feature = "smartstring")]
use smartstring::alias::String;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub version: u16,
    pub background_color: Color,
    pub transparent_nodes: bool,
    pub node_colors: HashMap<String, Color>,
}
