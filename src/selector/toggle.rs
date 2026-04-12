//! Form Control Components
//!
//! Provides Toggle (switch), Checkbox, Radio (radio button)
use dioxus::prelude::*;

// ==================== Toggle 切换开关 ====================

/// Toggle
#[derive(Props, PartialEq, Clone)]
pub struct ToggleProps {
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

/// Toggle component
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let class = props.class.unwrap_or_default();

    let active = if props.checked {
        "nd-toggle-active"
    } else {
        ""
    };

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        "cursor: pointer;"
    };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            "aria-checked": if props.checked { "true" } else { "false" },
            disabled: if props.disabled { "true" } else { "false" },
            class: "nd-toggle {active} {class}",
            style: disabled_style,
            onclick: move |_| {
                if !props.disabled {
                    props.on_change.call(!props.checked);
                }
            },
            div {
                class: "nd-toggle-thumb",
            }
        }
    }
}
