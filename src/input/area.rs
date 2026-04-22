use dioxus::prelude::*;

/// TextArea Props
#[derive(Props, PartialEq, Clone)]
pub struct TextAreaProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Inline style
    #[props(default)]
    pub style: Option<String>,

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
    /// Name Attribute
    #[props(default)]
    pub name: Option<String>,
    /// Placeholder Text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Number of Rows (initial height)
    #[props(default = 3)]
    pub rows: u32,
    /// TextArea Value
    pub value: String,

    /// Input Event
    pub oninput: EventHandler<String>,
}

/// TextArea Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     div {
///         label { "Message" }
///         TextArea {
///             placeholder: "Your message...",
///             rows: 4,
///             value: message,
///             oninput: move |val| set_message(val),
///         }
///     }
/// }
/// ```
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        textarea {
            class: "nd-surface-inset nd-shadow-inset nd-input {class}",
            style: props.style,
            disabled: props.disabled,
            id: props.id,
            maxlength: props.maxlength,
            name: props.name,
            placeholder: props.placeholder,
            resize: "none",
            rows: props.rows,
            value: props.value,
            "data-error": props.error,
            oninput: move |evt| {
                props.oninput.call(evt.value().clone());
            }
        }
    }
}
