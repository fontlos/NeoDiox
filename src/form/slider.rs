use dioxus::prelude::*;

/// Slider
#[derive(Props, PartialEq, Clone)]
pub struct SliderProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Minimum value
    #[props(default = 0)]
    pub min: i32,
    /// Maximum value
    #[props(default = 100)]
    pub max: i32,
    /// Step size
    #[props(default = 1)]
    pub step: i32,
    /// Value
    pub value: i32,

    /// Change event
    pub onchange: EventHandler<i32>,
}

/// Slider Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Slider {
///         value: volume,
///         onchange: move |val| set_volume(val),
///     }
/// }
/// ```
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let class = props.class.unwrap_or_default();

    // 计算百分比
    let percentage = if props.max != props.min {
        ((props.value - props.min) as f64 / (props.max - props.min) as f64) * 100.0
    } else {
        0.0
    };

    rsx! {
        div { class: "nd-slider {class}",

            // 轨道背景
            div { class: "nd-slider-track nd-surface-inset nd-shadow-inset" }

            // 进度条
            div { class: "nd-slider-progress", style: "width: {percentage}%;" }

            // 滑块
            div {
                class: "nd-slider-thumb nd-surface nd-shadow-sm",
                style: "left: {percentage}%;",
            }

            // 实际输入
            input {
                class: "nd-slider-input",
                r#type: "range",
                value: props.value,
                min: props.min,
                max: props.max,
                step: props.step,
                disabled: props.disabled,
                "aria-valuemin": props.min,
                "aria-valuemax": props.max,
                "aria-valuenow": props.value,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<i32>() {
                        props.onchange.call(val);
                    }
                },
            }
        }
    }
}
