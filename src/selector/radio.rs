use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,
    /// ID for label
    #[props(default)]
    pub id: Option<String>,
    /// Name of the radio group
    pub name: String,
    /// Value of this radio item
    pub value: String,

    /// Change event
    pub onchange: EventHandler<String>,
}

/// Radio component
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-radio {class}",
            input {
                r#type: "radio",
                id: props.id,
                name: props.name,
                value: "{props.value}",
                checked: props.checked,
                disabled: props.disabled,
                class: "nd-radio-input",
                onchange: move |_| {
                    props.onchange.call(props.value.clone());
                },
            }
        }
    }
}
