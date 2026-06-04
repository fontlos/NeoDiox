use dioxus::prelude::*;

use std::fmt::Display;

use crate::icon;

/// MultiSelect Props
#[derive(Props, PartialEq, Clone)]
pub struct MultiSelectProps<T: Display + PartialEq + Clone + 'static> {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// MultiSelect Options
    pub options: Vec<T>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Current selected values
    pub values: Vec<T>,

    /// Change event
    pub onchange: EventHandler<Vec<T>>,
}

/// MultiSelect Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     MultiSelect {
///         options: vec![
///             "JavaScript",
///             "TypeScript",
///         ],
///         placeholder: "Select skills...".to_string(),
///         values: selected_skills.clone(),
///         onchange: move |vals| set_selected_skills(vals),
///     }
/// }
/// ```
#[component]
pub fn MultiSelect<T: Display + PartialEq + Clone + 'static>(props: MultiSelectProps<T>) -> Element {
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
                                "data-selected": props.values.contains(&option),

                                onmousedown: move |evt| {
                                    evt.prevent_default();
                                },
                                onclick: move |_| {
                                    let mut new_values = values.clone();
                                    if new_values.contains(&option) {
                                        new_values.retain(|val| val != &option);
                                    } else {
                                        new_values.push(option.clone());
                                    }
                                    props.onchange.call(new_values);
                                },

                                "{option}"
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
                    if let Some(option) = props.options.iter().find(|o| o == &value) {
                        span {
                            class: "nd-multiselect-tag",
                            "{option}"
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
