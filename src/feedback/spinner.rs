use dioxus::prelude::*;

/// Spinner Loading
#[derive(Props, PartialEq, Clone)]
pub struct SpinnerProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Size (pixels)
    #[props(default = 32)]
    pub size: u32,
}

/// Spinner component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Spinner { }
/// }
/// ```
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let class = props.class.unwrap_or_default();
    let size = props.size;
    rsx! {
        div {
            class: "nd-spinner {class}",
            "aria-label": "Loading",
            style: "width: {size}px; height: {size}px;",
        }
    }
}
