use dioxus::prelude::*;

/// Skeleton
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonProps {
    /// Width (CSS value)
    #[props(default = "100%".to_string())]
    pub width: String,
    /// Height (CSS value)
    #[props(default = "16px".to_string())]
    pub height: String,
    /// Border radius (pixels)
    #[props(default = 8)]
    pub border_radius: u32,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Skeleton {
///         width: "200px".to_string(),
///         height: "20px".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-skeleton {class}",
            style: format!(
                "width: {}; height: {}; border-radius: {}px;",
                props.width, props.height, props.border_radius
            ),
        }
    }
}
