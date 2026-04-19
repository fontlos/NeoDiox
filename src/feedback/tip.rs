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
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,

    /// Tip Content
    pub content: String,
    /// Tip Position
    #[props(default)]
    pub position: TipPosition,

    /// Trigger Element
    pub children: Element,
}

/// Tip Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Tip {
///         content: "Home".to_string(),
///         position: TipPosition::Top,
///         Badge { "Hover me" }
///     }
/// }
/// ```
#[component]
pub fn Tip(props: TipProps) -> Element {
    let position_class = match props.position {
        TipPosition::Top => "nd-tip-top",
        TipPosition::Bottom => "nd-tip-bottom",
        TipPosition::Left => "nd-tip-left",
        TipPosition::Right => "nd-tip-right",
    };

    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-tip-container {class}",

            // 触发元素
            {props.children}

            // 提示内容
            div {
                role: "tip",
                class: "nd-tip {position_class}",
                "{props.content}"
            }
        }
    }
}
