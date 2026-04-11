//! Theme Context
//!
//! Provides theme state management

use dioxus::prelude::*;

/// Theme Variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    #[default]
    Light,
    Dark,
}

impl ThemeVariant {
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }
}

/// Theme Configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeConfig {
    pub variant: ThemeVariant,
    pub bg_primary: String,
    pub bg_secondary: String,
    pub shadow_dark: String,
    pub shadow_light: String,
    pub primary: String,
    pub primary_dark: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::light()
    }
}

impl ThemeConfig {
    pub fn light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            bg_primary: "#e6e9ef".to_string(),
            bg_secondary: "#d1d5db".to_string(),
            shadow_dark: "#b8bcc2".to_string(),
            shadow_light: "#ffffff".to_string(),
            primary: "#7c3aed".to_string(),
            primary_dark: "#6d28d9".to_string(),
        }
    }

    pub fn dark() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            bg_primary: "#374151".to_string(),
            bg_secondary: "#1f2937".to_string(),
            shadow_dark: "#111827".to_string(),
            shadow_light: "#4b5563".to_string(),
            primary: "#7c3aed".to_string(),
            primary_dark: "#6d28d9".to_string(),
        }
    }
}

pub fn use_theme_config() -> ThemeConfig {
    let theme = use_context::<Signal<ThemeConfig>>();
    theme.read().clone()
}

pub fn use_toggle_theme() -> impl FnMut() {
    let mut theme = use_context::<Signal<ThemeConfig>>();

    move || {
        let new_variant = match theme.read().variant {
            ThemeVariant::Light => ThemeVariant::Dark,
            ThemeVariant::Dark => ThemeVariant::Light,
        };
        *theme.write() = match new_variant {
            ThemeVariant::Light => ThemeConfig::light(),
            ThemeVariant::Dark => ThemeConfig::dark(),
        };
    }
}
