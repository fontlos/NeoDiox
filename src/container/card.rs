use crate::container::NeuFlat;
use dioxus::prelude::*;

/// Card
#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    /// Whether hover effect
    #[props(default = true)]
    pub hoverable: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Card Content
    pub children: Element,
}

/// Card Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Card {
///         "Lightning fast performance"
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        NeuFlat {
            border_radius: 16,
            class: "nd-card {class}",
            style: if props.hoverable {
                "transition: all 0.3s ease;"
            } else {
                ""
            },
            {props.children}
        }
    }
}
