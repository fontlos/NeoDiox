use dioxus::prelude::*;

/// DatePicker Props
#[derive(Props, PartialEq, Clone)]
pub struct DatePickerProps {
    /// Current selected date (ISO format)
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
    /// Minimum date
    #[props(default)]
    pub min: Option<String>,
    /// Maximum date
    #[props(default)]
    pub max: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// DatePicker Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     div {
///         label { "Select Date" }
///         DatePicker {
///             value: selected_date,
///             on_change: move |val| set_selected_date(Some(val)),
///         }
///     }
/// }
/// ```
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let value = props.value.clone().unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let placeholder = props.placeholder.clone().unwrap_or_default();

    rsx! {
        div {
            class: "nd-date-picker {class}",

            div {
                class: "nd-date-picker-wrapper",

                input {
                    r#type: "date",
                    value,
                    placeholder,
                    disabled: if props.disabled { "true" } else { "false" },
                    min: props.min.clone().unwrap_or_default(),
                    max: props.max.clone().unwrap_or_default(),
                    class: "nd-date-picker-input nd-input-bg",
                    oninput: move |evt| {
                        let val = evt.value();
                        if !val.is_empty() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 日历图标
                span {
                    class: "nd-date-picker-icon",
                    "📅"
                }
            }
        }
    }
}
