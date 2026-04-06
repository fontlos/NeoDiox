//! Skeuomorphic Surface Components
//!
//! Provide basic skeuomorphic surfaces: Raised, Inset, Flat

use dioxus::prelude::*;

use crate::theme::{ThemeConfig, use_theme_config};

/// Skeuomorphic surface type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceType {
    #[default]
    Raised,
    RaisedSm,
    Inset,
    Flat,
}

// 生成拟物化表面样式
fn neu_surface_style(surface_type: SurfaceType, theme: &ThemeConfig) -> String {
    match surface_type {
        SurfaceType::Raised => format!(
            "background: linear-gradient(145deg, {}, {}); box-shadow: 8px 8px 16px {}, -8px -8px 16px {};",
            theme.bg_primary, theme.bg_secondary, theme.shadow_dark, theme.shadow_light
        ),
        SurfaceType::RaisedSm => format!(
            "box-shadow: 4px 4px 8px {}, -4px -4px 8px {};",
            theme.shadow_dark, theme.shadow_light
        ),
        SurfaceType::Inset => format!(
            "background: linear-gradient(145deg, {}, {}); box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
            theme.bg_secondary, theme.bg_primary, theme.shadow_dark, theme.shadow_light
        ),
        SurfaceType::Flat => format!(
            "background: linear-gradient(145deg, {}, {});",
            theme.bg_primary, theme.bg_secondary
        ),
    }
}

/// Skeuomorphic Surface
#[derive(Props, PartialEq, Clone)]
pub struct NeuSurfaceProps {
    /// Surface Type
    #[props(default)]
    pub surface_type: SurfaceType,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Inline style (will be merged with surface style)
    #[props(default)]
    pub style: Option<String>,
    /// Border radius
    #[props(default = 16)]
    pub border_radius: u32,
    /// Children
    pub children: Element,
}

/// Skeuomorphic Surface Component
///
/// Provide three skeuomorphic surface effects:
/// - `Raised`: Raised effect, used for cards and containers
/// - `RaisedSm`: Small raised effect, used for badges and small elements
/// - `Inset`: Inset effect, used for input fields and pressed states
/// - `Flat`: Flat effect, used for accordion panels and other elements
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     NeuSurface {
///         surface_type: SurfaceType::Raised,
///         "Raised Card Content"
///     }
/// }
/// ```
#[component]
pub fn NeuSurface(props: NeuSurfaceProps) -> Element {
    let theme = use_theme_config();

    let base_style = neu_surface_style(props.surface_type, &theme);
    let additional_style = props.style.unwrap_or_default();

    let combined_style = if additional_style.is_empty() {
        base_style
    } else {
        format!("{} {}", base_style, additional_style)
    };

    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-surface {class}",
            style: "{combined_style} border-radius: {props.border_radius}px;",
            {props.children}
        }
    }
}

/// Raised Surface Component
#[component]
pub fn NeuRaised(
    children: Element,
    #[props(default)] class: Option<String>,
    #[props(default)] style: Option<String>,
    #[props(default = 16)] border_radius: u32,
) -> Element {
    rsx! {
        NeuSurface {
            surface_type: SurfaceType::Raised,
            class,
            style,
            border_radius,
            {children}
        }
    }
}

/// Small-sized raised surface component
#[component]
pub fn NeuRaisedSm(
    children: Element,
    #[props(default)] class: Option<String>,
    #[props(default)] style: Option<String>,
    #[props(default = 12)] border_radius: u32,
) -> Element {
    rsx! {
        NeuSurface {
            surface_type: SurfaceType::RaisedSm,
            class,
            style,
            border_radius,
            {children}
        }
    }
}

/// Inset Surface Component
#[component]
pub fn NeuInset(
    children: Element,
    #[props(default)] class: Option<String>,
    #[props(default)] style: Option<String>,
    #[props(default = 12)] border_radius: u32,
) -> Element {
    rsx! {
        NeuSurface {
            surface_type: SurfaceType::Inset,
            class,
            style,
            border_radius,
            {children}
        }
    }
}

/// Flat Surface Component
#[component]
pub fn NeuFlat(
    children: Element,
    #[props(default)] class: Option<String>,
    #[props(default)] style: Option<String>,
    #[props(default = 12)] border_radius: u32,
) -> Element {
    rsx! {
        NeuSurface {
            surface_type: SurfaceType::Flat,
            class,
            style,
            border_radius,
            {children}
        }
    }
}
