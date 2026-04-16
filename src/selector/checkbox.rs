use dioxus::prelude::*;

use crate::icon;

/// Checkbox
#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,

    /// Change event
    pub onchange: EventHandler<bool>,
}

/// Checkbox component
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let class = props.class.unwrap_or_default();
    let disabled = props.disabled;
    let checked = props.checked;

    rsx! {
        div {
            class: "nd-checkbox {class}",
            icon::Icon {
                class: "nd-checkbox-icon",
                size: 12,
                color: "white",
                icon::Checked { }
            }
            input {
                r#type: "checkbox",
                checked,
                disabled,
                class: "nd-checkbox-input",
                onchange: move |_| {
                    props.onchange.call(!checked);
                },
            }
        }
    }
}
