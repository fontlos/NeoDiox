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

/// MultiSelect Component - 使用neu-inset容器 + neu-raised下拉菜单
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
    let is_open = use_signal(|| false);
    let class = props.class.unwrap_or_default();
    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Select...".to_string());

    rsx! {
        div {
            class: "nd-multiselect {class}",
            style: "position: relative;",

            // 容器/触发器 - neu-inset背景
            div {
                class: "nd-multiselect-container nd-input-bg",
                onclick: {
                    let mut is_open = is_open.clone();
                    move |_| {
                        let current = *is_open.read();
                        *is_open.write() = !current;
                    }
                },
                role: "listbox",
                "aria-multiselectable": "true",

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
                        "{placeholder}"
                    }
                }
            }

            // 下拉箭头按钮
            button {
                r#type: "button",
                class: "nd-multiselect-arrow",
                class: if *is_open.read() { "nd-multiselect-arrow-open" } else { "" },
                "aria-expanded": if *is_open.read() { "true" } else { "false" },
                "aria-haspopup": "listbox",
                onmousedown: move |evt| {
                    evt.prevent_default();
                },
                onclick: {
                    let mut is_open = is_open.clone();
                    move |_| {
                        let current = *is_open.read();
                        *is_open.write() = !current;
                    }
                },
                "▼"
            }

            // 下拉列表
            if *is_open.read() {
                {
                    let options = props.options.clone();
                    let values = props.values.clone();
                    let on_change = props.on_change.clone();

                    rsx! {
                        div {
                            class: "nd-multiselect-menu",
                            role: "listbox",

                            for option in options {
                                {
                                    let is_selected = values.contains(&option.value);
                                    let option_label = option.label.clone();
                                    let option_value = option.value.clone();
                                    let option_disabled = option.disabled;
                                    let values = values.clone();
                                    let on_change = on_change.clone();
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            role: "option",
                                            class: "nd-multiselect-item",
                                            class: if is_selected { "nd-multiselect-item-selected" } else { "" },
                                            "aria-selected": if is_selected { "true" } else { "false" },
                                            disabled: if option_disabled { "true" } else { "false" },
                                            onmousedown: move |evt| {
                                                evt.prevent_default();
                                            },
                                            onclick: move |_| {
                                                let mut new_values = values.clone();
                                                if new_values.contains(&option_value) {
                                                    new_values.retain(|val| val != &option_value);
                                                } else {
                                                    new_values.push(option_value.clone());
                                                }
                                                on_change.call(new_values);
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

                                            "{option_label}"
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
