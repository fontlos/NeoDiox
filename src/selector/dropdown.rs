use crate::surfaces::NeuRaised;
use dioxus::prelude::*;

/// Dropdown Option
#[derive(Clone, Debug, PartialEq)]
pub struct DropdownOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// Dropdown
#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    /// Dropdown Options
    pub options: Vec<DropdownOption>,
    /// Current selected value
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Whether to enable search
    #[props(default)]
    pub searchable: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Dropdown Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Dropdown {
///         options: vec![
///             DropdownOption { value: "cn".to_string(), label: "China".to_string(), disabled: false },
///             DropdownOption { value: "us".to_string(), label: "United States".to_string(), disabled: false },
///         ],
///         value: selected_country,
///         on_change: move |val| set_selected_country(val),
///         label: Some("Select Country".to_string()),
///     }
/// }
/// ```
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);
    let class = props.class.unwrap_or_default();

    let placeholder = props.placeholder.unwrap_or_else(|| "Select...".to_string());

    // 获取当前选中的标签
    let selected_label = props
        .value
        .as_ref()
        .and_then(|v| props.options.iter().find(|o| &o.value == v))
        .map(|o| o.label.clone())
        .unwrap_or(placeholder.clone());

    // 过滤选项
    let filtered_options: Vec<_> = props
        .options
        .iter()
        .filter(|o| {
            if o.disabled {
                return false;
            }
            if props.searchable && !search_query.read().is_empty() {
                return o
                    .label
                    .to_lowercase()
                    .contains(&search_query.read().to_lowercase());
            }
            true
        })
        .cloned()
        .collect();
    let filtered_is_empty = filtered_options.is_empty();

    rsx! {
        div {
            class: "nd-dropdown {class}",
            style: "position: relative;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit; margin-bottom: 8px; display: block;",
                    "{label_text}"
                }
            }

            // 触发器
            button {
                r#type: "button",
                class: "nd-dropdown-trigger",
                disabled: if props.disabled { "true" } else { "false" },
                style: format!(
                    "width: 100%; padding: 12px 40px 12px 16px; border-radius: 12px; \
                     font-size: 14px; color: {}; background: transparent; \
                     border: none; cursor: pointer; text-align: left; \
                     transition: all 0.2s ease; outline: none; position: relative;",
                    if props.value.is_none() { "inherit" } else { "inherit" }
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
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
                    ),
                }

                span {
                    style: if props.value.is_none() { "opacity: 0.6;" } else { "" },
                    "{selected_label}"
                }

                // 下拉箭头
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%); transition: transform 0.2s ease;",
                    if *is_open.read() { "▼" } else { "▼" }
                }
            }

            // 下拉列表
            if *is_open.read() {
                NeuRaised {
                    border_radius: 12,
                    class: "nd-dropdown-menu",
                    style: "position: absolute; top: 100%; left: 0; right: 0; \
                            margin-top: 8px; z-index: 50; max-height: 200px; \
                            overflow-y: auto;",

                    // 搜索框（如果启用）
                    if props.searchable {
                        div {
                            style: "padding: 8px; border-bottom: 1px solid rgba(128, 128, 128, 0.2);",
                            input {
                                r#type: "text",
                                value: "{search_query}",
                                placeholder: "Search...",
                                style: "width: 100%; padding: 8px; border: none; \
                                        background: transparent; color: inherit; \
                                        outline: none; font-size: 14px;",
                                oninput: move |evt| {
                                    search_query.write().clone_from(&evt.value());
                                },
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                },
                            }
                        }
                    }

                    // 选项列表
                    div {
                        style: "padding: 4px;",
                        for option in filtered_options {
                            button {
                                r#type: "button",
                                class: "nd-dropdown-item",
                                style: format!(
                                    "width: 100%; padding: 12px 16px; text-align: left; \
                                     background: none; border: none; cursor: pointer; \
                                     font-size: 14px; color: inherit; border-radius: 8px; \
                                     transition: background 0.15s ease;"
                                ),
                                onmouseenter: move |_| {
                                    // hover 效果
                                },
                                onclick: {
                                    let option_value = option.value.clone();
                                    let on_change = props.on_change.clone();
                                    let mut is_open = is_open.clone();
                                    let mut search_query = search_query.clone();
                                    move |_| {
                                        on_change.call(option_value.clone());
                                        *is_open.write() = false;
                                        search_query.write().clear();
                                    }
                                },
                                "{option.label}"
                            }
                        }

                        if filtered_is_empty {
                            div {
                                style: "padding: 12px 16px; text-align: center; \
                                        opacity: 0.6; font-size: 14px;",
                                "No options found"
                            }
                        }
                    }
                }
            }

            // 点击外部关闭
            if *is_open.read() {
                div {
                    style: "position: fixed; inset: 0; z-index: 40;",
                    onclick: {
                        let mut is_open = is_open.clone();
                        let mut search_query = search_query.clone();
                        move |_| {
                            *is_open.write() = false;
                            search_query.write().clear();
                        }
                    },
                }
            }
        }
    }
}
