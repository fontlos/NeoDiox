use dioxus::prelude::*;

/// RangeSlider
#[derive(Props, PartialEq, Clone)]
pub struct RangeSliderProps {
    /// Value
    pub value: i32,
    /// Change event
    pub on_change: EventHandler<i32>,
    /// Minimum value
    #[props(default = 0)]
    pub min: i32,
    /// Maximum value
    #[props(default = 100)]
    pub max: i32,
    /// Step size
    #[props(default = 1)]
    pub step: i32,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to show the value
    #[props(default = true)]
    pub show_value: bool,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// RangeSlider Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     RangeSlider {
///         value: volume,
///         on_change: move |val| set_volume(val),
///         label: Some("Volume".to_string()),
///         min: 0,
///         max: 100,
///     }
/// }
/// ```
#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    let class = props.class.unwrap_or_default();

    // 计算百分比
    let percentage = if props.max != props.min {
        ((props.value - props.min) as f64 / (props.max - props.min) as f64) * 100.0
    } else {
        0.0
    };

    rsx! {
        div {
            class: "nd-range-slider {class}",
            style: "display: flex; flex-direction: column; gap: 12px;",

            // 标签和值
            if props.label.is_some() || props.show_value {
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    if let Some(label_text) = props.label {
                        span {
                            style: "font-size: 14px; font-weight: 500; color: inherit;",
                            "{label_text}"
                        }
                    }
                    if props.show_value {
                        span {
                            style: "font-size: 14px; font-weight: 600; color: #7c3aed;",
                            "{props.value}"
                        }
                    }
                }
            }

            // 滑块容器
            div {
                style: "position: relative; height: 12px; border-radius: 6px;",

                // 轨道背景
                div {
                    class: "nd-range-slider-track",
                }

                // 进度条
                div {
                    class: "nd-range-slider-progress",
                    style: "width: {percentage}%;",
                }

                // 实际输入（隐藏但可交互）
                input {
                    r#type: "range",
                    value: props.value,
                    min: props.min,
                    max: props.max,
                    step: props.step,
                    disabled: if props.disabled { "true" } else { "false" },
                    "aria-valuemin": props.min,
                    "aria-valuemax": props.max,
                    "aria-valuenow": props.value,
                    style: "position: absolute; inset: 0; width: 100%; \
                            height: 100%; opacity: 0; cursor: pointer; z-index: 4;",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<i32>() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 自定义滑块thumb - 添加 z-index: 3 确保在最上方
                div {
                    class: "nd-range-slider-thumb",
                    style: "left: {percentage}%;",
                }
            }
        }
    }
}
