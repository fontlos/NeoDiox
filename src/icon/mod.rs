//! Icon Component
//!
//! Provides flexible icon support via:
//! - Class variant: Iconify, Font Awesome, or any CSS-based icon library
//! - Element variant: Any Element (SVG, img, etc.)

mod icon;
mod preset;

pub use icon::Icon;
pub use preset::{Checked, Close, Error, Info, Warning};
