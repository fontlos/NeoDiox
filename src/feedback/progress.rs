use dioxus::prelude::*;

/// Progress Bar
#[derive(Props, PartialEq, Clone)]
pub struct ProgressBarProps {
    /// Current progress value (0-100)
    #[props(default = 0)]
    pub value: u8,
    /// Height of the progress bar in pixels
    #[props(default = 12)]
    pub height: u32,
    /// Custom class name for styling
    #[props(default)]
    pub class: Option<String>,
}

/// Progress Bar component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     ProgressBar {
///         value: progress,
///     }
/// }
/// ```
#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-progress-bar {class}",
            role: "progressbar",
            "aria-valuenow": props.value,
            "aria-valuemin": 0,
            "aria-valuemax": 100,

            // 进度条轨道
            div {
                class: "nd-progress-bar-track",
                style: format!(
                    "height: {}px;",
                    props.height
                ),

                // 进度填充 - 紫色渐变
                div {
                    class: "nd-progress-bar-fill",
                    style: format!(
                        "width: {}%;",
                        props.value
                    ),
                    // Shimmer 效果, 来自骨架屏
                    div {
                        class: "shimmer",
                    }
                }
            }
        }
    }
}
