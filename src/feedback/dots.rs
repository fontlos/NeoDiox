use dioxus::prelude::*;

/// Dots Props
#[derive(Props, PartialEq, Clone)]
pub struct DotsProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Dots Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Dots { }
/// }
/// ```
#[component]
pub fn Dots(props: DotsProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-dots {class}",
            role: "status",
            "aria-label": "Loading",

            for _ in 0..3 {
                div { class: "nd-dots-dot" }
            }
        }
    }
}
