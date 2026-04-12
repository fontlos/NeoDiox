//! Label Components
//!
//! Provides Badge, Tooltip

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

// ==================== Tooltip 提示 ====================

/// Tooltip Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Tooltip
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    /// Tooltip Content
    pub content: String,
    /// Trigger Element
    pub children: Element,
    /// Tooltip Position
    #[props(default)]
    pub position: TooltipPosition,
    /// Whether to show the tooltip
    pub is_visible: bool,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Tooltip Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Tooltip {
///         content: "Home".to_string(),
///         is_visible: show_tooltip,
///         Button {
///             onclick: move |_| {},
///             "🏠"
///         }
///     }
/// }
/// ```
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let position_style = match props.position {
        TooltipPosition::Top => {
            "bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 8px;"
        }
        TooltipPosition::Bottom => {
            "top: 100%; left: 50%; transform: translateX(-50%); margin-top: 8px;"
        }
        TooltipPosition::Left => {
            "right: 100%; top: 50%; transform: translateY(-50%); margin-right: 8px;"
        }
        TooltipPosition::Right => {
            "left: 100%; top: 50%; transform: translateY(-50%); margin-left: 8px;"
        }
    };

    // 箭头样式
    let arrow_style = match props.position {
        TooltipPosition::Top => {
            "top: 100%; left: 50%; transform: translateX(-50%); \
            border: 6px solid transparent; border-top-color: #1f2937;"
        }
        TooltipPosition::Bottom => {
            "bottom: 100%; left: 50%; transform: translateX(-50%); \
            border: 6px solid transparent; border-bottom-color: #1f2937;"
        }
        TooltipPosition::Left => {
            "left: 100%; top: 50%; transform: translateY(-50%); \
            border: 6px solid transparent; border-left-color: #1f2937;"
        }
        TooltipPosition::Right => {
            "right: 100%; top: 50%; transform: translateY(-50%); \
            border: 6px solid transparent; border-right-color: #1f2937;"
        }
    };
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-tooltip-container {class}",

            // 触发元素
            {props.children}

            // 提示内容
            if props.is_visible {
                div {
                    role: "tooltip",
                    style: format!(
                        "position: absolute; {position_style} \
                         padding: 8px 12px; border-radius: 8px; \
                         font-size: 12px; font-weight: 500; \
                         color: white; background: #1f2937; \
                         white-space: nowrap; z-index: 100; \
                         pointer-events: none;",
                    ),
                    "{props.content}"

                    // 箭头
                    div {
                        style: format!(
                            "position: absolute; {arrow_style}"
                        ),
                    }
                }
            }
        }
    }
}
