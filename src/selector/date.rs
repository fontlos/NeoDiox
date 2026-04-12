use dioxus::prelude::*;

/// DatePicker
#[derive(Props, PartialEq, Clone)]
pub struct DatePickerProps {
    /// Current selected date (ISO format)
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Placeholder text
    #[props(default = "Select a date".to_string())]
    pub placeholder: String,
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
///     DatePicker {
///         value: selected_date,
///         on_change: move |val| set_selected_date(Some(val)),
///         label: Some("Select Date".to_string()),
///     }
/// }
/// ```
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let value = props.value.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-date-picker {class}",
            style: "display: flex; flex-direction: column; gap: 8px;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit;",
                    "{label_text}"
                }
            }

            div {
                style: "position: relative;",

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 12px; z-index: -1; \
                         background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
                    ),
                }

                input {
                    r#type: "date",
                    value,
                    disabled: if props.disabled { "true" } else { "false" },
                    min: props.min.unwrap_or_default(),
                    max: props.max.unwrap_or_default(),
                    style: format!(
                        "width: 100%; padding: 12px 40px 12px 16px; border-radius: 12px; \
                         font-size: 14px; color: inherit; background: transparent; \
                         border: none; outline: none; cursor: pointer;"
                    ),
                    oninput: move |evt| {
                        let val = evt.value();
                        if !val.is_empty() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 日历图标
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%); pointer-events: none; \
                            font-size: 18px; opacity: 0.6;",
                    "📅"
                }
            }
        }
    }
}
