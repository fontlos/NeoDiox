use dioxus::prelude::*;

/// DotsLoading Props
#[derive(Props, PartialEq, Clone)]
pub struct DotsLoadingProps {
    /// Dot size in pixels
    #[props(default = 8)]
    pub size: u32,
    /// Dot color
    #[props(default = "#7c3aed".to_string())]
    pub color: String,
    /// Gap between dots
    #[props(default = 8)]
    pub gap: u32,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// DotsLoading Component - 三个弹跳点的加载动画
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     DotsLoading {
///         size: 8,
///         color: "#7c3aed".to_string(),
///     }
/// }
/// ```
#[component]
pub fn DotsLoading(props: DotsLoadingProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-dots {class}",
            style: format!("gap: {}px;", props.gap),
            role: "status",
            "aria-label": "Loading",

            for _ in 0..3 {
                div {
                    class: "nd-dots-dot",
                    style: format!(
                        "width: {}px; height: {}px; background: {};",
                        props.size, props.size, props.color
                    ),
                }
            }
        }
    }
}
