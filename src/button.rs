//! Button Component
//!
//! Provides skeuomorphic buttons in various variants and states

use crate::theme::use_theme;
use dioxus::prelude::*;

/// Button Variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Default Neuromorphic Style
    #[default]
    Neuromorphic,
    /// Custom Gradient Style (color1, color2, font_color)
    Gradient(&'static str, &'static str, &'static str),
}

impl ButtonVariant {
    pub const PRIMARY: Self =
        Self::Gradient("var(--nd-primary)", "var(--nd-primary-dark)", "#ffffff");
    pub const SUCCESS: Self = Self::Gradient("#22c55e", "#16a34a", "#ffffff");
    pub const DANGER: Self = Self::Gradient("#ef4444", "#dc2626", "#ffffff");
    pub const WARNING: Self = Self::Gradient("#eab308", "#ca8a04", "#ffffff");
    pub const INFO: Self = Self::Gradient("#3b82f6", "#2563eb", "#ffffff");
}

/// Button
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
    /// Inline style
    #[props(default)]
    pub style: Option<String>,
    /// Button Variants
    #[props(default)]
    pub variant: ButtonVariant,
    /// Button Width (CSS Value)
    #[props(default)]
    pub width: Option<String>,
    /// Button Height (CSS Value)
    #[props(default)]
    pub height: Option<String>,
    /// Button Padding (CSS Value)
    #[props(default = "12px 24px".to_string())]
    pub padding: String,
    /// Border Radius (CSS Value)
    #[props(default = "12px".to_string())]
    pub border_radius: String,
    /// Whether the button is disabled
    #[props(default)]
    pub disabled: bool,
    /// Button Type
    #[props(default = "button".to_string())]
    pub r#type: String,

    /// Click Event
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Child Elements
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

    // TODO: 可以考虑把静态 CSS 分离出去, 内联也许会对子元素造成覆盖
    let color = match props.variant {
        ButtonVariant::Neuromorphic => {
            let text_color = if theme.is_dark() {
                "#d1d5db"
            } else {
                "#4b5563"
            };
            format!(
                "background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); box-shadow: 8px 8px 16px var(--nd-shadow-dark), -8px -8px 16px var(--nd-shadow-light); color: {};",
                text_color
            )
        }
        ButtonVariant::Gradient(color1, color2, font_color) => format!(
            "background: linear-gradient(145deg, {}, {}); box-shadow: 4px 4px 8px var(--nd-shadow-dark), -4px -4px 8px var(--nd-shadow-light); color: {};",
            color1, color2, font_color
        ),
    };
    let style = format!(
        "{color}{}{}padding: {};border-radius: {};{}",
        props
            .width
            .map(|w| format!("width: {};", w))
            .unwrap_or_default(),
        props
            .height
            .map(|h| format!("height: {};", h))
            .unwrap_or_default(),
        props.padding,
        props.border_radius,
        props.style.unwrap_or_default()
    );

    let disabled = props.disabled;
    let r#type = props.r#type;

    rsx! {
        button {
            class: "nd-btn {class}",
            style,
            disabled,
            r#type,
            onclick: move |evt| {
                if let Some(handler) = props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
