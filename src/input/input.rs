use dioxus::prelude::*;

// ==================== TextInput 文本输入 ====================

/// TextInput
#[derive(Props, PartialEq, Clone)]
pub struct TextInputProps {
    /// TextInput Value
    pub value: String,
    /// Input Event
    pub on_input: EventHandler<String>,
    /// Placeholder Text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label Text
    #[props(default)]
    pub label: Option<String>,
    /// Is Required
    #[props(default)]
    pub required: bool,
    /// Is Disabled
    #[props(default)]
    pub disabled: bool,
    /// Is Read Only
    #[props(default)]
    pub read_only: bool,
    /// Input Type (text, password, email, etc.)
    #[props(default = "text".to_string())]
    pub input_type: String,
    /// Max Length
    #[props(default)]
    pub max_length: Option<u32>,
    /// Error Message
    #[props(default)]
    pub error: Option<String>,
    /// Help Text
    #[props(default)]
    pub help_text: Option<String>,
    /// Autocomplete Attribute
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Input Size
    #[props(default)]
    pub size: InputSize,
    /// Is Clearable
    #[props(default)]
    pub clearable: bool,
}

/// Input Size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl InputSize {
    pub fn padding(&self) -> &'static str {
        match self {
            Self::Small => "8px 12px",
            Self::Medium => "12px 16px",
            Self::Large => "16px 20px",
        }
    }

    pub fn font_size(&self) -> &'static str {
        match self {
            Self::Small => "12px",
            Self::Medium => "14px",
            Self::Large => "16px",
        }
    }
}

/// TextInput Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     TextInput {
///         value: name,
///         on_input: move |val| set_name(val),
///         label: Some("Full Name".to_string()),
///         required: true,
///         placeholder: Some("John Doe".to_string()),
///     }
/// }
/// ```
#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let placeholder = props.placeholder.unwrap_or_default();
    let error_id = format!("{}-error", "input");
    let class = props.class.unwrap_or_default();
    let has_error = props.error.is_some();

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };

    let padding = props.size.padding();
    let font_size = props.size.font_size();
    let clear_padding = if props.clearable { "40px" } else { "16px" };

    rsx! {
        div {
            class: "nd-text-input {class}",

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

            // 输入框容器
            div { class: "nd-text-input-wrapper",
                input {
                    r#type: "{props.input_type}",
                    value: "{props.value}",
                    placeholder,
                    disabled: if props.disabled { "true" } else { "false" },
                    readonly: if props.read_only { "true" } else { "false" },
                    required: if props.required { "true" } else { "false" },
                    maxlength: props.max_length,
                    autocomplete: props.autocomplete.unwrap_or_default(),
                    "aria-invalid": if has_error { "true" } else { "false" },
                    "aria-describedby": if has_error { Some(error_id.clone()) } else { None },
                    class: "nd-input nd-input-bg",
                    style: format!(
                        "padding: {padding}; padding-right: {clear_padding}; \
                         border-radius: 12px; font-size: {font_size}; \
                         {}",
                        disabled_style,
                    ),
                    oninput: move |evt| {
                        props.on_input.call(evt.value().clone());
                    },
                }

                // 清除按钮
                if props.clearable && !props.value.is_empty() && !props.disabled {
                    button {
                        r#type: "button",
                        class: "nd-input-clear",
                        onclick: move |_| {
                            props.on_input.call(String::new());
                        },
                        "✕"
                    }
                }
            }

            // 错误信息
            if let Some(ref error_text) = props.error {
                p {
                    id: "{error_id}",
                    role: "alert",
                    class: "nd-error",
                    "{error_text}"
                }
            }

            // 辅助文本
            if let Some(ref help_text) = props.help_text {
                if !has_error {
                    p { class: "nd-help", "{help_text}" }
                }
            }
        }
    }
}
