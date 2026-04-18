use dioxus::prelude::*;

use crate::button::ButtonVariant;

// 这里复用了 Button 的变体和 class 类

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
    // 复用自 button
    let (mut class, color) = match props.variant {
        ButtonVariant::Neuromorphic => ("nd-surface nd-shadow-sm nd-text-soft ".to_string(), "".to_string()),
        ButtonVariant::Gradient(c1, c2, f) => (
            "nd-shadow-sm ".to_string(),
            format!("background: linear-gradient(145deg, {c1}, {c2}); color: {f};"),
        ),
    };

    if let Some(c) = props.class {
        class.push_str(&c);
    }

    rsx! {
        span {
            class: "nd-badge {class}",
            style: color,
            {props.children}
        }
    }
}
