//! Button Component
//!
//! Provides skeuomorphic buttons in various variants and states

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
    let (mut class, color) = match props.variant {
        ButtonVariant::Neuromorphic => {
            // 预留空格
            ("nd-surface nd-shadow nd-text-soft ".to_string(), "".to_string())
        }
        ButtonVariant::Gradient(c1, c2, f) => (
            "nd-shadow-sm ".to_string(),
            format!("background: linear-gradient(145deg, {c1}, {c2}); color: {f};"),
        ),
    };

    if let Some(c) = props.class {
        class.push_str(&c);
    }

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
