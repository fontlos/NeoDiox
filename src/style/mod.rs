use dioxus::prelude::*;

macro_rules! include_strs {
    ($file:expr) => { include_str!($file) };
    ($file:expr, $($files:expr),+) => {
        concat!(include_str!($file), "\n", include_strs!($($files),+))
    };
}

/// Core Styles. Generate by build.rs
pub const STYLE: Asset = asset!("./style.css");
/// Normalize CSS.
pub const RESET: Asset = asset!("./normalize.css");

/// Core Styles as String. Generate in compile time
pub const STYLE_STR: &'static str = include_strs!(
    "base.css",
    "container.css",
    "input.css",
    "selector.css",
    "navigation.css",
    "feedback.css",
    "data.css",
    "advanced.css"
);
/// Normalize CSS as String.
pub const RESET_STR: &'static str = include_str!("normalize.css");
