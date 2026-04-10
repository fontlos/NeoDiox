use dioxus::prelude::*;

pub const STYLE: Asset = asset!("./style.css");
pub const NORMALIZE: Asset = asset!("./normalize.css");

pub const STYLE_S: &'static str = include_str!("./style.css");
pub const NORMALIZE_S: &'static str = include_str!("./normalize.css");
