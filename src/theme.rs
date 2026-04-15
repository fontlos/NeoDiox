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

    pub fn theme(&self) -> &'static str {
        match self {
            Self::Light => "",
            Self::Dark => "nd-dark",
        }
    }
}

/// Use to OVERWRITE the default theme variables
pub struct ThemeVars {
    /// Text color
    pub text: &'static str,
    /// Text soft color
    pub text_soft: &'static str,
    /// Background primary color
    pub bg_primary: &'static str,
    /// Background secondary color
    pub bg_secondary: &'static str,
    /// Shadow dark color
    pub shadow_dark: &'static str,
    /// Shadow light color
    pub shadow_light: &'static str,
    /// Dark Text color
    pub dark_text: &'static str,
    /// Dark Text soft color
    pub dark_text_soft: &'static str,
    /// Dark background primary color
    pub dark_bg_primary: &'static str,
    /// Dark background secondary color
    pub dark_bg_secondary: &'static str,
    /// Dark shadow dark color
    pub dark_shadow_dark: &'static str,
    /// Dark shadow light color
    pub dark_shadow_light: &'static str,
    /// Primary color
    pub primary: &'static str,
    /// Primary dark color
    pub primary_dark: &'static str,
}

impl Default for ThemeVars {
    fn default() -> Self {
        Self {
            text: "#000000",
            text_soft: "#4b5563",
            bg_primary: "#e6e9ef",
            bg_secondary: "#d1d5db",
            shadow_dark: "#b8bcc2",
            shadow_light: "#ffffff",
            dark_text: "#ffffff",
            dark_text_soft: "#d1d5db",
            dark_bg_primary: "#374151",
            dark_bg_secondary: "#1f2937",
            dark_shadow_dark: "#111827",
            dark_shadow_light: "#4b5563",
            primary: "#7c3aed",
            primary_dark: "#6d28d9",
        }
    }
}

impl ThemeVars {
    pub fn to_css(&self) -> String {
        format!(
            ":root{{
    --nd-text:{};
    --nd-text-soft:{};
    --nd-bg-primary:{};
    --nd-bg-secondary:{};
    --nd-shadow-dark:{};
    --nd-shadow-light:{};
    --nd-primary:{};
    --nd-primary-dark:{}
}}
.nd-dark{{
    --nd-text:{};
    --nd-text-soft:{};
    --nd-bg-primary:{};
    --nd-bg-secondary:{};
    --nd-shadow-dark:{};
    --nd-shadow-light:{}
}}",
            self.text,
            self.text_soft,
            self.bg_primary,
            self.bg_secondary,
            self.shadow_dark,
            self.shadow_light,
            self.primary,
            self.primary_dark,
            self.dark_text,
            self.dark_text_soft,
            self.dark_bg_primary,
            self.dark_bg_secondary,
            self.dark_shadow_dark,
            self.dark_shadow_light,
        )
    }
}

pub fn use_init_theme(theme: ThemeVariant) {
    let theme = use_signal(|| theme);
    use_context_provider(|| theme);
}

pub fn use_theme() -> Signal<ThemeVariant> {
    use_context::<Signal<ThemeVariant>>()
}

pub fn use_toggle_theme() -> impl FnMut() {
    let mut theme = use_context::<Signal<ThemeVariant>>();

    move || {
        let new_variant = match *theme.read() {
            ThemeVariant::Light => ThemeVariant::Dark,
            ThemeVariant::Dark => ThemeVariant::Light,
        };
        *theme.write() = new_variant;
    }
}
