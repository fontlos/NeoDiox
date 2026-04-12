use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,
    /// Value of this radio button (used for grouping)
    pub value: String,
    /// Change event (passes the value of this radio button)
    pub on_change: EventHandler<String>,
    /// Label text (optional)
    #[props(default)]
    pub label: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Radio component
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let class = props.class.unwrap_or_default();

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };

    // Dot opacity animation
    let dot_opacity = if props.checked { "1" } else { "0" };

    // If has label, render Radio + Label (button before text)
    if let Some(label_text) = props.label {
        return rsx! {
            div {
                class: "nd-radio-with-label {class}",
                button {
                    r#type: "button",
                    role: "radio",
                    "aria-checked": if props.checked { "true" } else { "false" },
                    disabled: if props.disabled { "true" } else { "false" },
                    class: "nd-radio nd-radio-bg",
                    style: disabled_style,
                    onclick: move |_| {
                        if !props.disabled {
                            props.on_change.call(props.value.clone());
                        }
                    },
                    // Dot - opacity transition
                    div {
                        class: "nd-radio-dot",
                        style: format!("opacity: {};", dot_opacity),
                    }
                }
                label {
                    class: "nd-label",
                    "{label_text}"
                }
            }
        };
    }

    rsx! {
        button {
            r#type: "button",
            role: "radio",
            "aria-checked": if props.checked { "true" } else { "false" },
            disabled: if props.disabled { "true" } else { "false" },
            class: "nd-radio nd-radio-bg {class}",
            style: disabled_style,
            onclick: move |_| {
                if !props.disabled {
                    props.on_change.call(props.value.clone());
                }
            },
            div {
                class: "nd-radio-dot",
                style: format!("opacity: {};", dot_opacity),
            }
        }
    }
}

/// Radio option for RadioGroup
#[derive(Clone, PartialEq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// RadioGroup
#[derive(Props, PartialEq, Clone)]
pub struct RadioGroupProps {
    /// Current selected value
    pub value: String,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Group label (optional)
    #[props(default)]
    pub legend: Option<String>,
    /// Radio options
    pub options: Vec<RadioOption>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// RadioGroup component
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        fieldset {
            class: "nd-radio-group {class}",
            if let Some(legend_text) = props.legend {
                legend {
                    class: "nd-label",
                    style: "margin-bottom: 12px;",
                    "{legend_text}"
                }
            }
            div {
                role: "radiogroup",
                class: "nd-radio-options",
                for option in &props.options {
                    Radio {
                        checked: props.value == option.value,
                        value: option.value.clone(),
                        disabled: option.disabled,
                        label: Some(option.label.clone()),
                        on_change: props.on_change.clone(),
                    }
                }
            }
        }
    }
}
