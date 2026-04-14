use dioxus::prelude::*;

/// TimePicker Props
#[derive(Props, PartialEq, Clone)]
pub struct TimePickerProps {
    /// Current selected time (HH:MM format)
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// TimePicker Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     div {
///         label { "Select Time" }
///         TimePicker {
///             value: selected_time,
///             on_change: move |val| set_selected_time(Some(val)),
///         }
///     }
/// }
/// ```
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let value = props.value.clone().unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let placeholder = props.placeholder.clone().unwrap_or_default();

    rsx! {
        div {
            class: "nd-time-picker {class}",

            div {
                class: "nd-time-picker-wrapper",

                input {
                    r#type: "time",
                    value,
                    placeholder,
                    disabled: if props.disabled { "true" } else { "false" },
                    class: "nd-time-picker-input nd-input-bg",
                    oninput: move |evt| {
                        let val = evt.value();
                        if !val.is_empty() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 时钟图标
                span {
                    class: "nd-time-picker-icon",
                    "🕐"
                }
            }
        }
    }
}
