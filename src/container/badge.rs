use dioxus::prelude::*;

use crate::button::ButtonVariant;
use crate::theme::use_theme;

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
    let theme = use_theme();
    let class = props.class.unwrap_or_default();

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

    rsx! {
        span {
            class: "nd-badge {class}",
            style: color,
            {props.children}
        }
    }
}
