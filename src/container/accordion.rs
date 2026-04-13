use crate::container::NeuFlat;

use dioxus::prelude::*;

// TODO: 基于 grid 和 padding 的动画, 也许需要更改
/// Accordion
#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Title
    pub title: String,
    /// Content
    pub content: String,
    /// Initially expanded
    #[props(default)]
    pub expanded: bool,
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
///     }
/// }
/// ```
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut expanded = use_signal(|| props.expanded);
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
                    onclick: move |_| {
                        expanded.set(!expanded());
                    },
                    span {
                        class: "nd-accordion-title",
                        "{props.title}"
                    }
                    span {
                        class: "nd-accordion-icon",
                        class: if expanded() { "nd-accordion-icon-expanded" } else { "" },
                        "▼"
                    }
                }

                div {
                    role: "region",
                    class: "nd-accordion-panel",
                    class: if expanded() { "nd-accordion-panel-expanded" } else { "" },

                    div {
                        class: "nd-accordion-content",
                        "{props.content}"
                    }
                }
            }
        }
    }
}
