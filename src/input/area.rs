use dioxus::prelude::*;

/// TextArea
#[derive(Props, PartialEq, Clone)]
pub struct TextAreaProps {
    /// TextArea Value
    pub value: String,
    /// Input Event
    pub on_input: EventHandler<String>,
    /// Placeholder Text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label Text
    #[props(default)]
    pub label: Option<String>,
    /// Number of Rows
    #[props(default = 3)]
    pub rows: u32,
    /// Is Required
    #[props(default)]
    pub required: bool,
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
    /// Is Resizeable
    #[props(default = ResizeMode::Vertical)]
    pub resize: ResizeMode,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Resize Mode for TextArea
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeMode {
    None,
    Horizontal,
    Vertical,
    Both,
}

impl std::fmt::Display for ResizeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
            Self::Both => write!(f, "both"),
        }
    }
}

/// TextArea Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     TextArea {
///         value: message,
///         on_input: move |val| set_message(val),
///         label: Some("Message".to_string()),
///         rows: 4,
///         placeholder: Some("Your message...".to_string()),
///     }
/// }
/// ```
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let placeholder = props.placeholder.unwrap_or_default();
    let class = props.class.unwrap_or_default();
    let has_error = props.error.is_some();

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };
    let resize_style = props.resize.to_string();

    rsx! {
        div {
            class: "nd-textarea {class}",

            // 标签
            if let Some(label_text) = props.label {
                label {
                    class: "nd-label",
                    "{label_text}"
                    if props.required {
                        span { class: "nd-label-required", "*" }
                    }
                }
            }

            // 文本区域
            textarea {
                value: "{props.value}",
                placeholder,
                rows: props.rows,
                disabled: if props.disabled { "true" } else { "false" },
                readonly: if props.read_only { "true" } else { "false" },
                required: if props.required { "true" } else { "false" },
                maxlength: props.max_length,
                "aria-invalid": if has_error { "true" } else { "false" },
                class: "nd-textarea-element nd-input-bg",
                style: format!(
                    "padding: 12px 16px; border-radius: 12px; font-size: 14px; \
                     resize: {resize_style}; {}",
                    disabled_style
                ),
                oninput: move |evt| {
                    props.on_input.call(evt.value().clone());
                },
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
