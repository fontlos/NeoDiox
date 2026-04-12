use dioxus::prelude::*;

/// Spinner Loading
#[derive(Props, PartialEq, Clone)]
pub struct SpinnerProps {
    /// Size (pixels)
    #[props(default = 24)]
    pub size: u32,
    /// Color
    #[props(default = "#7c3aed".to_string())]
    pub color: String,
    /// Border width
    #[props(default = 2)]
    pub border_width: u32,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Spinner component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Spinner {
///         size: 32,
///         color: "#7c3aed".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-spinner {class}",
            "aria-label": "Loading",
            style: format!(
                "width: {}px; height: {}px; border-width: {}px; color: {};",
                props.size, props.size, props.border_width, props.color
            ),
        }
    }
}
