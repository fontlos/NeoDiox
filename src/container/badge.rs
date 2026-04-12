use dioxus::prelude::*;

use crate::button::ButtonVariant;
use crate::theme::use_theme;

// ==================== Badge 徽章 ====================

/// Badge
#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    /// Badge Variants
    #[props(default)]
    pub variant: ButtonVariant,
    /// Badge Text
    pub label: String,
    /// Icon (Optional)
    #[props(default)]
    pub icon: Option<String>,
    /// Whether to show the status dot
    #[props(default)]
    pub show_dot: bool,
    /// Status dot color
    #[props(default)]
    pub dot_color: Option<String>,
    /// Whether the badge is dismissible
    #[props(default)]
    pub dismissible: bool,
    /// Dismiss Event
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Badge Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Badge {
///         variant: ButtonVariant::SUCCESS,
///         label: "Active".to_string(),
///         show_dot: true,
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
            style: format!(
                "display: inline-flex; align-items: center; gap: 6px; \
                 padding: 6px 12px; border-radius: 9999px; \
                 font-size: 12px; font-weight: 500; \
                 {color}"
            ),

            // // 状态点
            // if props.show_dot {
            //     span {
            //         style: format!(
            //             "width: 8px; height: 8px; border-radius: 50%; \
            //              background: {};",
            //             props.dot_color.unwrap_or_else(|| text_color.to_string())
            //         ),
            //     }
            // }

            // 图标
            if let Some(icon) = &props.icon {
                span {
                    class: "nd-badge-icon",
                    "{icon}"
                }
            }

            // 文本
            span {
                "{props.label}"
            }

            // 关闭按钮
            if props.dismissible {
                button {
                    r#type: "button",
                    class: "nd-badge-dismiss",
                    onclick: move |_| {
                        if let Some(handler) = props.on_dismiss {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}
