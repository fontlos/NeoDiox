use dioxus::prelude::*;

/// Skeleton
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Border radius (pixels)
    #[props(default = 8)]
    pub border_radius: u32,
    /// Height (CSS value)
    #[props(default = "16px".to_string())]
    pub height: String,
    /// Width (CSS value)
    #[props(default = "100%".to_string())]
    pub width: String,
}

/// Skeleton component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Skeleton { }
/// }
/// ```
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let class = props.class.unwrap_or_default();

    let border_radius = props.border_radius;
    let height = props.height;
    let width = props.width;

    rsx! {
        div {
            class: "nd-skeleton {class}",
            width: "{width}",
            height: "{height}",
            border_radius: "{border_radius}px",
        }
    }
}
