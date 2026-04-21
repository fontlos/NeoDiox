use dioxus::prelude::*;

/// Dropdown Option
#[derive(Clone, Debug, PartialEq)]
pub struct DropdownOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// Dropdown Props
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
    /// Whether to enable search
    #[props(default)]
    pub searchable: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Dropdown Component - 使用neu-inset输入框 + neu-raised下拉菜单
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Dropdown {
///         options: vec![
///             DropdownOption { value: "us".to_string(), label: "🇺🇸 United States".to_string(), disabled: false },
///             DropdownOption { value: "uk".to_string(), label: "🇬🇧 United Kingdom".to_string(), disabled: false },
///         ],
///         value: country,
///         on_change: move |val| set_country(val),
///         searchable: true,
///         placeholder: "Search countries...".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut input_value = use_signal(String::new);
    let class = props.class.unwrap_or_default();
    let placeholder = props
        .placeholder
        .clone()
        .unwrap_or_else(|| "Select...".to_string());

    // 同步外部value到input_value
    let sync_value = props.value.clone();
    let sync_options = props.options.clone();
    use_effect(move || {
        if let Some(ref v) = sync_value {
            if let Some(opt) = sync_options.iter().find(|o| &o.value == v) {
                *input_value.write() = opt.label.clone();
            }
        } else {
            input_value.write().clear();
        }
    });

    rsx! {
        div {
            class: "nd-dropdown {class}",
            style: "position: relative;",

            // 输入框/触发器
            div { class: "nd-dropdown-input-wrapper",
                input {
                    r#type: "text",
                    value: "{input_value}",
                    placeholder: "{placeholder}",
                    readonly: if !props.searchable { "true" } else { "false" },
                    class: "nd-dropdown-input nd-surface-inset nd-shadow-inset",
                    "aria-expanded": if *is_open.read() { "true" } else { "false" },
                    "aria-haspopup": "listbox",
                    autocomplete: "off",
                    onclick: {
                        let mut is_open = is_open.clone();
                        move |_| {
                            if !props.searchable {
                                let current = *is_open.read();
                                *is_open.write() = !current;
                            }
                        }
                    },
                    onfocus: {
                        let mut is_open = is_open.clone();
                        move |_| {
                            if props.searchable {
                                *is_open.write() = true;
                            }
                        }
                    },
                    oninput: move |evt| {
                        let val = evt.value().clone();
                        *input_value.write() = val.clone();
                        if props.searchable {
                            *is_open.write() = true;
                        }
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            *is_open.write() = false;
                        }
                    },
                }

                // 下拉箭头
                span {
                    class: "nd-dropdown-arrow",
                    class: if *is_open.read() { "nd-dropdown-arrow-open" } else { "" },
                    "▼"
                }
            }

            // 下拉列表
            if *is_open.read() {
                {
                    let search = input_value.read().clone();
                    let options = props.options.clone();
                    let current_value = props.value.clone();
                    let on_change = props.on_change.clone();
                    let is_searchable = props.searchable;
                    let filtered: Vec<_> = options
                        .iter()
                        .filter(|o| {
                            if o.disabled {
                                return false;
                            }
                            if is_searchable && !search.is_empty() {
                                return o.label.to_lowercase().contains(&search.to_lowercase());
                            }
                            true
                        })
                        .cloned()
                        .collect();
                    let is_empty = filtered.is_empty();

                    rsx! {
                        div {
                            class: "nd-dropdown-menu",
                            role: "listbox",

                            for option in filtered {
                                button {
                                    r#type: "button",
                                    role: "option",
                                    class: "nd-dropdown-item",
                                    class: if current_value.as_ref() == Some(&option.value) { "nd-dropdown-item-selected" } else { "" },
                                    onmousedown: move |evt| {
                                        evt.prevent_default();
                                    },
                                    onclick: {
                                        let option_label = option.label.clone();
                                        let option_value = option.value.clone();
                                        let on_change = on_change.clone();
                                        let mut input_value = input_value.clone();
                                        let mut is_open = is_open.clone();
                                        move |_| {
                                            *input_value.write() = option_label.clone();
                                            on_change.call(option_value.clone());
                                            *is_open.write() = false;
                                        }
                                    },
                                    "{option.label}"
                                }
                            }

                            if is_empty {
                                div {
                                    class: "nd-dropdown-empty",
                                    "No options found"
                                }
                            }
                        }
                    }
                }
            }

            // 点击外部关闭
            if *is_open.read() {
                div {
                    class: "nd-dropdown-backdrop",
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
