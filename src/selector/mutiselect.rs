use dioxus::prelude::*;

/// MultiSelect Option
#[derive(Clone, Debug, PartialEq)]
pub struct MultiSelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// MultiSelect Props
#[derive(Props, PartialEq, Clone)]
pub struct MultiSelectProps {
    /// MultiSelect Options
    pub options: Vec<MultiSelectOption>,
    /// Current selected values
    pub values: Vec<String>,
    /// Change event
    pub on_change: EventHandler<Vec<String>>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// MultiSelect Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     MultiSelect {
///         options: vec![
///             MultiSelectOption { value: "js".to_string(), label: "JavaScript".to_string(), disabled: false },
///             MultiSelectOption { value: "ts".to_string(), label: "TypeScript".to_string(), disabled: false },
///         ],
///         values: selected_skills.clone(),
///         on_change: move |vals| set_selected_skills(vals),
///         placeholder: "Select skills...".to_string(),
///     }
/// }
/// ```
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-multiselect {class}",
            style: "position: relative;",

            // 容器/触发器
            div {
                class: "nd-surface-inset nd-shadow-inset nd-multiselect-container",
                onclick: move |_| {
                    is_open.set(!is_open());
                },

                // 已选标签
                for value in &props.values {
                    if let Some(option) = props.options.iter().find(|o| &o.value == value) {
                        span {
                            class: "nd-multiselect-tag",
                            "{option.label}"
                            button {
                                r#type: "button",
                                class: "nd-multiselect-tag-remove",
                                onmousedown: move |evt| {
                                    evt.prevent_default();
                                    evt.stop_propagation();
                                },
                                onclick: {
                                    let value = value.clone();
                                    let values = props.values.clone();
                                    let on_change = props.on_change.clone();
                                    move |_| {
                                        let mut new_values = values.clone();
                                        new_values.retain(|v| v != &value);
                                        on_change.call(new_values);
                                    }
                                },
                                "✕"
                            }
                        }
                    }
                }

                // 占位符
                if props.values.is_empty() {
                    span {
                        class: "nd-multiselect-placeholder",
                        { props.placeholder }
                    }
                }
            }

            // 下拉箭头按钮
            button {
                r#type: "button",
                class: "nd-multiselect-arrow",
                class: if *is_open.read() { "nd-multiselect-arrow-open" } else { "" },
                onmousedown: move |evt| {
                    evt.prevent_default();
                },
                onclick: move |_|{
                    is_open.set(!is_open());
                },
                "▼"
            }

            // 下拉列表
            if *is_open.read() {
                {
                    let options = props.options.clone();

                    rsx! {
                        div {
                            class: "nd-multiselect-menu",
                            role: "listbox",

                            for option in options {
                                {
                                    let is_selected = props.values.contains(&option.value);
                                    let values = props.values.clone();
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            role: "option",
                                            class: "nd-multiselect-item",
                                            class: if is_selected { "nd-multiselect-item-selected" } else { "" },
                                            disabled: option.disabled,
                                            onmousedown: move |evt| {
                                                evt.prevent_default();
                                            },
                                            onclick: move |_| {
                                                let mut new_values = values.clone();
                                                if new_values.contains(&option.value) {
                                                    new_values.retain(|val| val != &option.value);
                                                } else {
                                                    new_values.push(option.value.clone());
                                                }
                                                props.on_change.call(new_values);
                                            },

                                            span {
                                                class: "nd-multiselect-check",
                                                class: if is_selected { "nd-multiselect-check-selected" } else { "" },
                                                if is_selected {
                                                    span {
                                                        class: "nd-multiselect-check-icon",
                                                        "✓"
                                                    }
                                                }
                                            }

                                            { option.label }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 点击外部关闭
            if *is_open.read() {
                div {
                    class: "nd-multiselect-backdrop",
                    onmousedown: move |evt| {
                        evt.prevent_default();
                    },
                    onclick: {
                        let mut is_open = is_open.clone();
                        move |_| {
                            *is_open.write() = false;
                        }
                    },
                }
            }
        }
    }
}
