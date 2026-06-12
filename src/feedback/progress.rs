use dioxus::prelude::*;

/// Progress Bar
#[derive(Props, PartialEq, Clone)]
pub struct ProgressBarProps {
    /// Custom class name for styling
    #[props(default)]
    pub class: Option<String>,

    /// Current progress value (0-100)
    #[props(default = 0)]
    pub value: u8,
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

    let value = props.value.clamp(0, 100);

    rsx! {
        div {
            class: "nd-progress-bar {class}",
            role: "progressbar",
            "aria-valuenow": value,
            "aria-valuemin": 0,
            "aria-valuemax": 100,

            // 进度条轨道
            div {
                class: "nd-progress-bar-track nd-surface-inset nd-shadow-inset",
                // 进度填充
                div {
                    class: "nd-progress-bar-fill",
                    style: "width: {value}%;",
                    // Shimmer 效果
                    div {
                        class: "nd-shimmer",
                    }
                }
            }
        }
    }
}
