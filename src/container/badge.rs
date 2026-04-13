use dioxus::prelude::*;

use crate::button::ButtonVariant;

// ==================== Badge 徽章 ====================

/// Badge
#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
    /// Badge Variants
    #[props(default)]
    pub variant: ButtonVariant,
    /// Children Elements
    pub children: Element,
}

/// Badge Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Badge {
///         variant: ButtonVariant::SUCCESS,
///         "Active"
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class = props.class.unwrap_or_default();

    let color = match props.variant {
        ButtonVariant::Neuromorphic => {
            "background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); box-shadow: 8px 8px 16px var(--nd-shadow-dark), -8px -8px 16px var(--nd-shadow-light); color: var(--nd-color-secondary);".to_string()
        }
        ButtonVariant::Gradient(color1, color2, font_color) => format!(
            "background: linear-gradient(145deg, {}, {}); box-shadow: 4px 4px 8px var(--nd-shadow-dark), -4px -4px 8px var(--nd-shadow-light); color: {};",
            color1, color2, font_color
        ),
    };

    rsx! {
        span {
            class: "nd-badge {class}",
            style: color,
            {props.children}
        }
    }
}
