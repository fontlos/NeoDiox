//! Skeuomorphic Surface Components
//!
//! Provide basic skeuomorphic surfaces: Raised, Inset, Flat

use dioxus::prelude::*;

/// Skeuomorphic surface type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceType {
    #[default]
    Raised,
    RaisedSm,
    Inset,
    Flat,
}

impl SurfaceType {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Raised => "nd-surface-raised",
            Self::RaisedSm => "nd-surface-raised-sm",
            Self::Inset => "nd-surface-inset",
            Self::Flat => "nd-surface-flat",
        }
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
    /// Additional inline style (for dynamic props only)
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
    let surface_class = props.surface_type.css_class();
    let class = props.class.unwrap_or_default();
    let combined_class = format!("nd-surface {surface_class} {class}");

    let style = props.style.unwrap_or_default();
    let border_radius_style = format!("border-radius: {}px;", props.border_radius);

    let combined_style = if style.is_empty() {
        border_radius_style
    } else {
        format!("{} {}", border_radius_style, style)
    };

    rsx! {
        div {
            class: combined_class,
            style: combined_style,
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
