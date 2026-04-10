//! Input Components
//!
//! Provides input controls such as TextInput, TextArea, SearchInput

use crate::theme::{ThemeConfig, use_theme_config};
use dioxus::prelude::*;

/// 输入框样式
fn input_style(theme: &ThemeConfig) -> String {
    format!(
        "background: linear-gradient(145deg, {}, {}); \
         box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
        theme.bg_secondary, theme.bg_primary, theme.shadow_dark, theme.shadow_light
    )
}

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

    let base_style = input_style(&use_theme_config());
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
            class: "neu-text-input {class}",

            // 标签
            if let Some(label_text) = props.label {
                label {
                    class: "neu-label",
                    "{label_text}"
                    if props.required {
                        span { class: "neu-label-required", "*" }
                    }
                }
            }

            // 输入框容器
            div { style: "position: relative;",
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
                    class: "neu-input",
                    style: format!(
                        "padding: {padding}; padding-right: {clear_padding}; \
                         border-radius: 12px; font-size: {font_size}; \
                         {} {} {}",
                        base_style, disabled_style, base_style
                    ),
                    oninput: move |evt| {
                        props.on_input.call(evt.value().clone());
                    },
                }

                // 清除按钮
                if props.clearable && !props.value.is_empty() && !props.disabled {
                    button {
                        r#type: "button",
                        class: "neu-input-clear",
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
                    class: "neu-error",
                    "{error_text}"
                }
            }

            // 辅助文本
            if let Some(ref help_text) = props.help_text {
                if !has_error {
                    p { class: "neu-help", "{help_text}" }
                }
            }
        }
    }
}

// ==================== TextArea 多行文本 ====================

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

    let base_style = input_style(&use_theme_config());
    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };
    let resize_style = props.resize.to_string();

    rsx! {
        div {
            class: "neu-textarea {class}",

            // 标签
            if let Some(label_text) = props.label {
                label {
                    class: "neu-label",
                    "{label_text}"
                    if props.required {
                        span { class: "neu-label-required", "*" }
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
                class: "neu-textarea-element",
                style: format!(
                    "padding: 12px 16px; border-radius: 12px; font-size: 14px; \
                     resize: {resize_style}; {} {}",
                    base_style, disabled_style
                ),
                oninput: move |evt| {
                    props.on_input.call(evt.value().clone());
                },
            }

            // 错误信息
            if let Some(ref error_text) = props.error {
                p {
                    role: "alert",
                    class: "neu-error",
                    "{error_text}"
                }
            }
        }
    }
}

// ==================== SearchInput 搜索输入 ====================

/// SearchInput
#[derive(Props, PartialEq, Clone)]
pub struct SearchInputProps {
    /// Search input value
    pub value: String,
    /// Input event
    pub on_input: EventHandler<String>,
    /// Placeholder text
    #[props(default = "Search...".to_string())]
    pub placeholder: String,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Is Disabled
    #[props(default)]
    pub disabled: bool,
    /// Is Clearable
    #[props(default = true)]
    pub clearable: bool,
    /// Is Auto-focused
    #[props(default)]
    pub autofocus: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Icon
    #[props(default)]
    pub search_icon: Option<String>,
}

/// SearchInput Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     SearchInput {
///         value: query,
///         on_input: move |val| set_query(val),
///         placeholder: "Search countries...".to_string(),
///     }
/// }
/// ```
#[component]
pub fn SearchInput(props: SearchInputProps) -> Element {
    let base_style = input_style(&use_theme_config());
    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };
    let search_icon = props.search_icon.unwrap_or_else(|| "🔍".to_string());
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-search-input {class}",
            style: "position: relative;",

            if let Some(label_text) = props.label {
                label {
                    class: "neu-label",
                    style: "margin-bottom: 8px; display: block;",
                    "{label_text}"
                }
            }

            div { style: "position: relative;",
                // 搜索图标
                span {
                    style: "position: absolute; left: 12px; top: 50%; \
                            transform: translateY(-50%); font-size: 16px; \
                            opacity: 0.6; pointer-events: none;",
                    "{search_icon}"
                }

                input {
                    r#type: "text",
                    value: "{props.value}",
                    placeholder: props.placeholder,
                    disabled: if props.disabled { "true" } else { "false" },
                    autofocus: if props.autofocus { "true" } else { "false" },
                    class: "neu-input",
                    style: format!(
                        "padding: 12px 40px; border-radius: 12px; font-size: 14px; \
                         {} {}",
                        base_style, disabled_style
                    ),
                    oninput: move |evt| {
                        props.on_input.call(evt.value().clone());
                    },
                }

                // 清除按钮
                if props.clearable && !props.value.is_empty() && !props.disabled {
                    button {
                        r#type: "button",
                        class: "neu-search-clear",
                        onclick: move |_| {
                            props.on_input.call(String::new());
                        },
                        "✕"
                    }
                }
            }
        }
    }
}
