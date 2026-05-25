use dioxus::prelude::*;

use crate::icon;

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
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// MultiSelect Options
    pub options: Vec<MultiSelectOption>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Current selected values
    pub values: Vec<String>,

    /// Change event
    pub onchange: EventHandler<Vec<String>>,
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
///         placeholder: "Select skills...".to_string(),
///         values: selected_skills.clone(),
///         onchange: move |vals| set_selected_skills(vals),
///     }
/// }
/// ```
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let class = props.class.unwrap_or_default();

    let mut is_open = use_signal(|| false);

    let menu = if is_open() {
        let options = props.options.clone();
        rsx! {
            div {
                class: "nd-multiselect-menu",

                for option in options {
                    {
                        let values = props.values.clone();
                        rsx! {
                            button {
                                class: "nd-multiselect-item",
                                "data-selected": props.values.contains(&option.value),

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
                                    props.onchange.call(new_values);
                                },

                                { option.label }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        div {
            class: "nd-multiselect {class}",
            "data-open": is_open,

            // 容器/触发器
            div {
                class: "nd-multiselect-container nd-surface-inset nd-shadow-inset",
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
                                class: "nd-multiselect-tag-remove",
                                onmousedown: move |evt| {
                                    evt.prevent_default();
                                    evt.stop_propagation();
                                },
                                onclick: {
                                    let value = value.clone();
                                    let values = props.values.clone();
                                    move |_| {
                                        let mut new_values = values.clone();
                                        new_values.retain(|v| v != &value);
                                        props.onchange.call(new_values);
                                    }
                                },
                                icon::Icon {
                                    class: "nd-multiselect-tag-remove",
                                    size: 12,
                                    icon::Close {}
                                }
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
                class: "nd-multiselect-arrow",
                onmousedown: move |evt| {
                    evt.prevent_default();
                },
                onclick: move |_| is_open.set(!is_open()),
                icon::Icon {
                    size: 16,
                    icon::Arrow {}
                }
            }

            // 下拉列表
            { menu }

            // 点击外部关闭
            if is_open() {
                div {
                    class: "nd-multiselect-backdrop",
                    onmousedown: move |evt| {
                        evt.prevent_default();
                    },
                    onclick: move |_| is_open.set(false),
                }
            }
        }
    }
}
