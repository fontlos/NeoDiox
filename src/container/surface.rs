//! Skeuomorphic Surface Components
//!
//! Provide basic skeuomorphic surfaces: Raised, Inset, Flat

use dioxus::prelude::*;

/// Skeuomorphic Surface
#[derive(Props, PartialEq, Clone)]
pub struct NeuSurfaceProps {
    /// Surface Type
    #[props(default)]
    pub surface_type: SurfaceType,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Additional inline style
    #[props(default)]
    pub style: Option<String>,
    /// Border radius
    #[props(default = 16)]
    pub border_radius: u32,
    /// Children
    pub children: Element,
}

/// Skeuomorphic surface type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SurfaceType {
    #[default]
    Raised,
    RaisedSm,
    Inset,
    Flat,
}

/// Skeuomorphic Surface Component
///
/// Provide three skeuomorphic surface effects:
/// - `Raised`: Raised effect, used for cards and containers
/// - `RaisedSm`: Small raised effect, used for badges and small elements
/// - `Inset`: Inset effect, used for input fields
/// - `Flat`: Flat effect, used for accordion panels and other elements
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     NeuSurface {
///         surface_type: SurfaceType::Raised,
///         "Raised Skeuomorphic Surface Content"
///     }
/// }
/// ```
#[component]
pub fn NeuSurface(props: NeuSurfaceProps) -> Element {
    let surface_class = match props.surface_type {
        SurfaceType::Raised => "nd-surface nd-shadow",
        SurfaceType::RaisedSm => "nd-surface nd-shadow-sm",
        SurfaceType::Inset => "nd-surface-inset nd-shadow-inset",
        SurfaceType::Flat => "nd-surface",
    };
    let class = props.class.unwrap_or_default();

    let style = props.style.unwrap_or_default();
    let border_radius = props.border_radius;

    rsx! {
        div {
            class: "{surface_class} {class}",
            style: "border-radius:{border_radius}px;{style}",
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
