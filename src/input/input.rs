use dioxus::prelude::*;

/// TextInput Props
#[derive(Props, PartialEq, Clone)]
pub struct TextInputProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Inline style
    #[props(default)]
    pub style: Option<String>,

    /// Autocomplete Attribute
    #[props(default)]
    pub autocomplete: Option<String>,
    /// Is Disabled
    #[props(default)]
    pub disabled: bool,
    /// Is Error State
    #[props(default)]
    pub error: bool,
    /// ID for label
    #[props(default)]
    pub id: Option<String>,
    /// Max Length
    #[props(default)]
    pub maxlength: Option<u32>,
    /// Placeholder Text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Is Read Only
    #[props(default)]
    pub readonly: bool,
    /// Input Type (text, password, email, etc.)
    #[props(default = "text".to_string())]
    pub r#type: String,
    /// Input Value
    pub value: String,

    /// Input Event
    pub oninput: EventHandler<String>,
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
///             error: name_error.is_some(),
///             placeholder: "Fontlos",
///             value: name,
///             oninput: move |val| set_name(val),
///         }
///     }
/// }
/// ```
#[component]
pub fn TextInput(props: TextInputProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        input {
            class: "nd-surface-inset nd-shadow-inset nd-input {class}",
            style: props.style,
            autocomplete: props.autocomplete,
            disabled: props.disabled,
            id: props.id,
            maxlength: props.maxlength,
            placeholder: props.placeholder,
            readonly: props.readonly,
            r#type: props.r#type,
            value: props.value,
            "data-error": props.error,
            oninput: move |evt| {
                props.oninput.call(evt.value().clone());
            }
        }
    }
}
