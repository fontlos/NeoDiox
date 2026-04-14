use dioxus::prelude::*;

/// TextInput Props
#[derive(Props, PartialEq, Clone)]
pub struct TextInputProps {
    /// Input Value
    pub value: String,
    /// Input Event
    pub on_input: EventHandler<String>,
    /// Placeholder Text
    #[props(default)]
    pub placeholder: Option<String>,
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
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Padding (default "12px 16px")
    #[props(default)]
    pub padding: Option<String>,
    /// Width (default "100%")
    #[props(default)]
    pub width: Option<String>,
    /// Is Clearable
    #[props(default)]
    pub clearable: bool,
    /// Autocomplete Attribute
    #[props(default)]
    pub autocomplete: Option<String>,
}

/// TextInput Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     div {
///         label { "Full Name" }
///         TextInput {
///             value: name,
///             on_input: move |val| set_name(val),
///             placeholder: "John Doe",
///         }
///     }
/// }
/// ```
#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let placeholder = props.placeholder.unwrap_or_default();
    let has_error = props.error.is_some();
    let class = props.class.unwrap_or_default();

    let padding = props.padding.unwrap_or_default();
    let width = props.width.unwrap_or_default();
    let clear_padding_right = if props.clearable { "40px" } else { "16px" };

    let input_style = {
        let mut s = String::new();
        if !width.is_empty() { s.push_str(&format!("width: {width}; ")); }
        if !padding.is_empty() { s.push_str(&format!("padding: {padding}; padding-right: {clear_padding_right}; ")); }
        else if props.clearable { s.push_str(&format!("padding-right: {clear_padding_right}; ")); }
        if props.disabled { s.push_str("opacity: 0.6; cursor: not-allowed; "); }
        if s.is_empty() { None } else { Some(s) }
    };

    rsx! {
        div {
            class: "nd-text-input {class}",

            div { class: "nd-text-input-wrapper",
                input {
                    r#type: "{props.input_type}",
                    value: "{props.value}",
                    placeholder,
                    disabled: if props.disabled { "true" } else { "false" },
                    readonly: if props.read_only { "true" } else { "false" },
                    maxlength: props.max_length,
                    autocomplete: props.autocomplete.unwrap_or_default(),
                    "aria-invalid": if has_error { "true" } else { "false" },
                    class: if has_error { "nd-input nd-input-bg nd-error-state" } else { "nd-input nd-input-bg" },
                    style: input_style,
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
                    role: "alert",
                    class: "nd-error",
                    "{error_text}"
                }
            }
        }
    }
}
