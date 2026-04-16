use dioxus::prelude::*;

/// Toggle
#[derive(Props, PartialEq, Clone)]
pub struct ToggleProps {
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

/// Toggle component
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-toggle {class}",
            input {
                r#type: "checkbox",
                checked: props.checked,
                disabled: props.disabled,
                class: "nd-toggle-input",
                onchange: move |_| {
                    props.onchange.call(!props.checked);
                },
            }
        }
    }
}
