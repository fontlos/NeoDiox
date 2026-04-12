use crate::surfaces::NeuRaised;
use dioxus::prelude::*;

/// MultiSelect Option
#[derive(Clone, Debug, PartialEq)]
pub struct MultiSelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// MultiSelect
#[derive(Props, PartialEq, Clone)]
pub struct MultiSelectProps {
    /// MultiSelect Options
    pub options: Vec<MultiSelectOption>,
    /// Current selected values
    pub values: Vec<String>,
    /// Change event
    pub on_change: EventHandler<Vec<String>>,
    /// Placeholder text
    #[props(default = "Select...".to_string())]
    pub placeholder: String,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
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
///         values: selected_skills,
///         on_change: move |vals| set_selected_skills(vals),
///         label: Some("Skills".to_string()),
///     }
/// }
/// ```
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let class = props.class.unwrap_or_default();
    let values = props.values.clone();
    let options = props.options.clone();
    let on_change = props.on_change.clone();

    let toggle_option = move |value: String, values: Vec<String>| {
        let mut values = values;
        if values.contains(&value) {
            values.retain(|v| v != &value);
        } else {
            values.push(value);
        }
        on_change.call(values);
    };

    rsx! {
        div {
            class: "nd-multiselect {class}",
            style: "position: relative;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit; margin-bottom: 8px; display: block;",
                    "{label_text}"
                }
            }

            // 触发器/容器
            button {
                r#type: "button",
                class: "nd-multiselect-trigger",
                disabled: if props.disabled { "true" } else { "false" },
                style: format!(
                    "width: 100%; min-height: 48px; padding: 8px 40px 8px 12px; \
                     border-radius: 12px; display: flex; flex-wrap: wrap; \
                     gap: 8px; align-items: center; cursor: pointer; \
                     transition: all 0.2s ease; outline: none; position: relative;"
                ),
                onclick: {
                    let mut is_open = is_open.clone();
                    move |_| {
                        if !props.disabled {
                            let current = *is_open.read();
                            *is_open.write() = !current;
                        }
                    }
                },

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 12px; z-index: -1; \
                         background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);",
                    ),
                }

                // 已选标签
                for value in &values {
                    if let Some(option) = options.iter().find(|o| &o.value == value) {
                        span {
                            style: "display: inline-flex; align-items: center; gap: 4px; \
                                    padding: 4px 8px; border-radius: 6px; font-size: 12px; \
                                    background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                    color: white; font-weight: 500;",
                            "{option.label}"
                            button {
                                r#type: "button",
                                style: "background: none; border: none; cursor: pointer; \
                                        color: white; padding: 0; font-size: 12px;",
                                onclick: {
                                    let value = value.clone();
                                    let values = values.clone();
                                    let toggle_option = toggle_option.clone();
                                    move |evt| {
                                        evt.stop_propagation();
                                        toggle_option(value.clone(), values.clone());
                                    }
                                },
                                "✕"
                            }
                        }
                    }
                }

                // 占位符
                if values.is_empty() {
                    span {
                        style: "opacity: 0.6; font-size: 14px;",
                        "{props.placeholder}"
                    }
                }

                // 下拉箭头
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%);",
                    "▼"
                }
            }

            // 下拉列表
            if *is_open.read() {
                NeuRaised {
                    border_radius: 12,
                    class: "nd-multiselect-menu",
                    style: "position: absolute; top: 100%; left: 0; right: 0; \
                            margin-top: 8px; z-index: 50; max-height: 200px; \
                            overflow-y: auto;",
                    div {
                        style: "padding: 4px;",
                        for option in &options {
                            button {
                                r#type: "button",
                                class: "nd-multiselect-item",
                                style: {
                                    let inset_style = format!(
                                        "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                                         box-shadow: inset 2px 2px 4px var(--nd-shadow-dark), inset -2px -2px 4px var(--nd-shadow-light);"
                                    );
                                    format!(
                                        "width: 100%; padding: 12px 16px; text-align: left; \
                                         background: none; border: none; cursor: pointer; \
                                         font-size: 14px; color: inherit; border-radius: 8px; \
                                         display: flex; align-items: center; gap: 8px; \
                                         transition: background 0.15s ease; \
                                         {}",
                                        if values.contains(&option.value) {
                                            "background: linear-gradient(145deg, #7c3aed, #6d28d9);"
                                        } else {
                                            &inset_style
                                        }
                                    )
                                },
                                disabled: if option.disabled { "true" } else { "false" },
                                onclick: {
                                    let option_value = option.value.clone();
                                    let values = values.clone();
                                    let toggle_option = toggle_option.clone();
                                    move |_| {
                                        toggle_option(option_value.clone(), values.clone());
                                    }
                                },

                                // 复选框
                                div {
                                    style: {
                                        let inset = format!(
                                            "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                                             box-shadow: inset 2px 2px 4px var(--nd-shadow-dark), inset -2px -2px 4px var(--nd-shadow-light);"
                                        );
                                        format!(
                                            "width: 16px; height: 16px; border-radius: 4px; \
                                             display: flex; align-items: center; justify-content: center; \
                                             {}",
                                            if values.contains(&option.value) {
                                                "background: linear-gradient(145deg, #7c3aed, #6d28d9);"
                                            } else {
                                                &inset
                                            }
                                        )
                                    },
                                    if values.contains(&option.value) {
                                        span {
                                            style: "color: white; font-size: 10px;",
                                            "✓"
                                        }
                                    }
                                }

                                "{option.label}"
                            }
                        }
                    }
                }

                // 点击外部关闭
                div {
                    style: "position: fixed; inset: 0; z-index: 40;",
                    onclick: move |_| {
                        *is_open.write() = false;
                    },
                }
            }
        }
    }
}
