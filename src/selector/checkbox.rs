use dioxus::prelude::*;

/// Checkbox
#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,
    /// Change event
    pub on_change: EventHandler<bool>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Checkbox component
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let class = props.class.unwrap_or_default();

    let active = if props.checked {
        "nd-checkbox-active"
    } else {
        ""
    };

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };

    // Checkmark opacity animation
    let check_opacity = if props.checked { "1" } else { "0" };

    rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            "aria-checked": if props.checked { "true" } else { "false" },
            disabled: if props.disabled { "true" } else { "false" },
            class: "nd-checkbox {active} {class}",
            style: disabled_style,
            onclick: move |_| {
                if !props.disabled {
                    props.on_change.call(!props.checked);
                }
            },
            // Checkmark icon - opacity transition
            span {
                class: "nd-checkbox-icon",
                style: format!("opacity: {};", check_opacity),
                "✓"
            }
        }
    }
}
