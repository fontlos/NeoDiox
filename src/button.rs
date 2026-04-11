//! Button Component
//!
//! Provides skeuomorphic buttons in various variants and states

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Button Variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Success,
    Danger,
    Warning,
    Info,
}

impl ButtonVariant {
    pub fn gradient(&self) -> (&'static str, &'static str) {
        match self {
            Self::Default => ("#6b7280", "#4b5563"),
            Self::Primary => ("#7c3aed", "#6d28d9"),
            Self::Success => ("#22c55e", "#16a34a"),
            Self::Danger => ("#ef4444", "#dc2626"),
            Self::Warning => ("#eab308", "#ca8a04"),
            Self::Info => ("#3b82f6", "#2563eb"),
        }
    }
}

/// Button
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    /// Button Variants
    #[props(default)]
    pub variant: ButtonVariant,
    /// Whether the button is disabled
    #[props(default)]
    pub disabled: bool,
    /// Button Type
    #[props(default = "button".to_string())]
    pub r#type: String,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
    /// Button Width (CSS Value)
    #[props(default)]
    pub width: Option<String>,
    /// Click Event
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Child Elements (Button Text or Content)
    pub children: Element,
}

/// Button Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Button {
///         variant: ButtonVariant::Primary,
///         onclick: move |_| println!("Clicked!"),
///         "Click Me"
///     }
/// }
/// ```
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let theme = use_theme();
    let class = props.class.unwrap_or_default();
    let is_gradient = props.variant != ButtonVariant::Default;

    // Only dynamic styles
    let dynamic_style = if is_gradient {
        let (color1, color2) = props.variant.gradient();
        format!(
            "background: linear-gradient(145deg, {}, {}); box-shadow: 4px 4px 8px var(--nd-shadow-dark), -4px -4px 8px var(--nd-shadow-light); color: #ffffff;",
            color1, color2
        )
    } else {
        let text_color = if theme.is_dark() {
            "#d1d5db"
        } else {
            "#4b5563"
        };
        format!(
            "background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); box-shadow: 8px 8px 16px var(--nd-shadow-dark), -8px -8px 16px var(--nd-shadow-light); color: {};",
            text_color
        )
    };

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };

    let width_style = props
        .width
        .map(|w| format!("width: {};", w))
        .unwrap_or_default();

    let combined_style = format!("{} {} {}", dynamic_style, disabled_style, width_style);

    // Handle click event
    let onclick = if props.disabled {
        None
    } else {
        props.onclick
    };

    rsx! {
        button {
            r#type: "{props.r#type}",
            class: "nd-btn {class}",
            disabled: if props.disabled { "true" } else { "false" },
            style: combined_style,
            onclick: move |evt| {
                if let Some(handler) = onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
