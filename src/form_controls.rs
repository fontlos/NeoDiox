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

// ==================== Checkbox 复选框 ====================

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

    // // Background style (theme-dependent)
    // let bg_style = if props.checked {
    //     "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
    //      box-shadow: 4px 4px 8px rgba(0,0,0,0.2);"
    // } else {
    //     "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
    //     box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
    // };

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

/// ==================== Radio 单选按钮 ====================

/// Radio
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

    // Always inset background (theme-dependent)
    let bg_style = format!(
        "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);",
    );

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
                    class: "nd-radio",
                    style: format!("{} {}", bg_style, disabled_style),
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
            class: "nd-radio {class}",
            style: format!("{} {}", bg_style, disabled_style),
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

// ==================== RadioGroup 单选组 ====================

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
