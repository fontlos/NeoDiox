use dioxus::prelude::*;

use std::fmt::Display;

use crate::icon;

/// Dropdown Props
#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps<T: Display + PartialEq + Clone + 'static> {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Dropdown Options
    pub options: Vec<T>,
    /// Current selected value
    #[props(default)]
    pub value: Option<T>,
    /// Change event
    pub onchange: EventHandler<T>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Whether to enable search
    #[props(default)]
    pub searchable: bool,
}

/// Dropdown Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Dropdown {
///         options: vec![
///             "USA".to_string(),
///             "Canada".to_string(),
///         ],
///         value: country,
///         onchange: move |val| set_country(val),
///         searchable: true,
///         placeholder: "Search countries...".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Dropdown<T: Display + PartialEq + Clone + 'static>(props: DropdownProps<T>) -> Element {
    let class = props.class.unwrap_or_default();

    let mut is_open = use_signal(|| false);
    let mut input = use_signal(|| {
        props.value.as_ref().map(|v| v.to_string()).unwrap_or_default()
    });

    rsx! {
        div {
            class: "nd-dropdown {class}",
            style: "position: relative;",

            div { class: "nd-dropdown-input-wrapper",
                input {
                    class: "nd-dropdown-input nd-surface-inset nd-shadow-inset nd-text",
                    value: "{input}",
                    placeholder: props.placeholder,
                    readonly: !props.searchable,
                    autocomplete: "off",
                    onclick: move |_| is_open.set(true),
                    onblur: move |_| is_open.set(false),
                    oninput: move |evt| input.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            is_open.set(false);
                        }
                    },
                }

                // 下拉箭头
                icon::Icon {
                    class: if is_open() { "nd-dropdown-arrow nd-dropdown-arrow-open" } else { "nd-dropdown-arrow" },
                    size: 16,
                    icon::Arrow {}
                }
            }

            // 下拉列表
            if is_open() {
                {
                    let search = input.read().clone();
                    let filtered: Vec<_> = props.options
                        .iter()
                        .filter(|o| {
                            if props.searchable && !search.is_empty() {
                                return o.to_string().to_lowercase().contains(&search.to_lowercase());
                            }
                            true
                        })
                        .cloned()
                        .collect();

                    rsx! {
                        div {
                            class: "nd-dropdown-menu",
                            onmousedown: |e| {
                                // 阻止冒泡, 防止选项点击还没触发就触发了 blur 导致菜单关闭
                                e.prevent_default();
                                e.stop_propagation();
                            },

                            for option in filtered {
                                button {
                                    class: if Some(&option) == props.value.as_ref()
                                        { "nd-dropdown-item selected" }
                                        else { "nd-dropdown-item" },
                                    onclick: {
                                        let option = option.clone();
                                        let onchange = props.onchange.clone();
                                        let mut is_open = is_open.clone();
                                        let mut input_value = input.clone();
                                        move |_| {
                                            input_value.set(option.to_string());  // 直接覆盖
                                            onchange.call(option.clone());
                                            is_open.set(false);
                                        }
                                    },
                                    "{option}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
