use dioxus::prelude::*;

/// Tip Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Tip
#[derive(Props, PartialEq, Clone)]
pub struct TipProps {
    /// Tip Content
    pub content: String,
    /// Trigger Element
    pub children: Element,
    /// Tip Position
    #[props(default)]
    pub position: TipPosition,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Tip Component - 使用 CSS hover 显示
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Tip {
///         content: "Home".to_string(),
///         position: TipPosition::Top,
///         Button {
///             onclick: move |_| {},
///             "🏠"
///         }
///     }
/// }
/// ```
#[component]
pub fn Tip(props: TipProps) -> Element {
    let position_class = match props.position {
        TipPosition::Top => "nd-tooltip-top",
        TipPosition::Bottom => "nd-tooltip-bottom",
        TipPosition::Left => "nd-tooltip-left",
        TipPosition::Right => "nd-tooltip-right",
    };

    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-tooltip-container {class}",

            // 触发元素
            {props.children}

            // 提示内容 - CSS hover 控制显示
            div {
                role: "tooltip",
                class: "nd-tooltip {position_class}",
                "{props.content}"
            }
        }
    }
}
