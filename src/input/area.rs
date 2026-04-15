use dioxus::prelude::*;

/// TextArea Props
#[derive(Props, PartialEq, Clone)]
pub struct TextAreaProps {
    /// TextArea Value
    pub value: String,
    /// Input Event
    pub on_input: EventHandler<String>,
    /// Placeholder Text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Number of Rows (initial height)
    #[props(default = 3)]
    pub rows: u32,
    /// Maximum Height (e.g. "200px")
    #[props(default)]
    pub max_height: Option<String>,
    /// Is Disabled
    #[props(default)]
    pub disabled: bool,
    /// Is Read Only
    #[props(default)]
    pub read_only: bool,
    /// Max Length
    #[props(default)]
    pub max_length: Option<u32>,
    /// Error Message
    #[props(default)]
    pub error: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// TextArea Component - 纯textarea元素，无label，由调用者控制标签
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     div {
///         label { "Message" }
///         TextArea {
///             value: message,
///             on_input: move |val| set_message(val),
///             rows: 4,
///             max_height: "200px".to_string(),
///             placeholder: "Your message...",
///         }
///     }
/// }
/// ```
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let placeholder = props.placeholder.unwrap_or_default();
    let has_error = props.error.is_some();
    let class = props.class.unwrap_or_default();
    let max_height = props
        .max_height
        .clone()
        .unwrap_or_else(|| "200px".to_string());

    let textarea_style = {
        let mut s = String::from("max-height: ");
        s.push_str(&max_height);
        s.push(';');
        if props.disabled {
            s.push_str(" opacity: 0.6; cursor: not-allowed;");
        }
        Some(s)
    };

    rsx! {
        div {
            class: "nd-textarea {class}",

            div { class: "nd-textarea-wrapper",
                textarea {
                    value: "{props.value}",
                    placeholder,
                    resize: "none",
                    rows: props.rows,
                    disabled: if props.disabled { "true" } else { "false" },
                    readonly: if props.read_only { "true" } else { "false" },
                    maxlength: props.max_length,
                    "aria-invalid": if has_error { "true" } else { "false" },
                    class: if has_error { "nd-textarea-element nd-input-bg nd-error-state" } else { "nd-textarea-element nd-input-bg" },
                    style: textarea_style,
                    oninput: move |evt| {
                        props.on_input.call(evt.value().clone());
                    },
                }
            }

            // 错误信息
            if let Some(ref error_text) = props.error {
                p {
                    role: "alert",
                    class: "nd-error",
                    "{error_text}"
                }
            }
        }
    }
}
