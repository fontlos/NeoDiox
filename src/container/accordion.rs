use crate::container::NeuFlat;

use dioxus::prelude::*;

/// Accordion
#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 是否展开
    #[props(default)]
    pub expanded: bool,
    /// 切换事件
    pub on_toggle: EventHandler<()>,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
}

/// Accordion Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Accordion {
///         title: "What is neuromorphic design?",
///         content: "Soft shadows and gradients...",
///         expanded: is_expanded,
///         on_toggle: move |_| toggle(),
///     }
/// }
/// ```
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-accordion {class}",

            NeuFlat {
                border_radius: 12,
                style: "overflow: hidden;",

                button {
                    r#type: "button",
                    class: "nd-accordion-trigger",
                    disabled: if props.disabled { "true" } else { "false" },
                    "aria-expanded": if props.expanded { "true" } else { "false" },
                    onclick: move |_| {
                        if !props.disabled {
                            props.on_toggle.call(());
                        }
                    },

                    span {
                        class: "nd-accordion-title",
                        "{props.title}"
                    }

                    span {
                        class: "nd-accordion-icon",
                        class: if props.expanded { "nd-accordion-icon-expanded" } else { "" },
                        "▼"
                    }
                }

                // 面板 - 始终渲染
                div {
                    role: "region",
                    class: "nd-accordion-panel",
                    class: if props.expanded { "nd-accordion-panel-expanded" } else { "" },
                    style: if props.expanded {
                        "max-height: 500px;"
                    } else {
                        "max-height: 0;"
                    },

                    div {
                        class: "nd-accordion-content",
                        "{props.content}"
                    }
                }
            }
        }
    }
}
