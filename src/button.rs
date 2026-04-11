//! Button Component
//!
//! Provides skeuomorphic buttons in various variants and states

use crate::theme::use_theme_config;
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
    /// Whether the button is loading
    #[props(default)]
    pub loading: bool,
    /// Text to display while loading
    #[props(default)]
    pub loading_text: Option<String>,
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
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();
    let is_gradient = props.variant != ButtonVariant::Default;

    // Dynamic styles only (theme-dependent background/shadow)
    let dynamic_style = if is_gradient {
        let (color1, color2) = props.variant.gradient();
        format!(
            "background: linear-gradient(145deg, {}, {}); \
             box-shadow: 4px 4px 8px {}, -4px -4px 8px {}; \
             color: #ffffff;",
            color1, color2, theme.shadow_dark, theme.shadow_light
        )
    } else {
        let text_color = if theme.variant.is_dark() {
            "#d1d5db"
        } else {
            "#4b5563"
        };
        format!(
            "background: linear-gradient(145deg, {}, {}); color: {}; \
             box-shadow: 8px 8px 16px {}, -8px -8px 16px {};",
            theme.bg_primary, theme.bg_secondary, text_color, theme.shadow_dark, theme.shadow_light
        )
    };

    let disabled_style = if props.disabled || props.loading {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };

    let width_style = props
        .width
        .as_ref()
        .map(|w| format!("width: {};", w))
        .unwrap_or_default();

    let combined_style = format!("{} {} {}", dynamic_style, disabled_style, width_style);

    // Handle click event
    let onclick = if props.disabled || props.loading {
        None
    } else {
        props.onclick
    };

    let content = if props.loading {
        let loading_text = props
            .loading_text
            .unwrap_or_else(|| "Loading...".to_string());
        rsx! {
            div { class: "flex items-center justify-center gap-2",
                div { class: "nd-spinner-inline" }
                span { "{loading_text}" }
            }
        }
    } else {
        rsx! { {props.children} }
    };

    rsx! {
        button {
            r#type: "{props.r#type}",
            class: "nd-btn {class}",
            disabled: if props.disabled || props.loading { "true" } else { "false" },
            style: combined_style,
            onclick: move |evt| {
                if let Some(handler) = onclick {
                    handler.call(evt);
                }
            },
            {content}
        }
    }
}
