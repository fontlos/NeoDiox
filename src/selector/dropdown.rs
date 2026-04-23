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
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Dropdown Options
    pub options: Vec<DropdownOption>,
    /// Current selected value
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub onchange: EventHandler<String>,
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
///             DropdownOption { value: "us".to_string(), label: "🇺🇸 United States".to_string(), disabled: false },
///             DropdownOption { value: "uk".to_string(), label: "🇬🇧 United Kingdom".to_string(), disabled: false },
///         ],
///         value: country,
///         onchange: move |val| set_country(val),
///         searchable: true,
///         placeholder: "Search countries...".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let class = props.class.unwrap_or_default();

    let mut is_open = use_signal(|| false);
    let mut input_value = use_signal(String::new);

    rsx! {
        div {
            class: "nd-dropdown {class}",
            style: "position: relative;",

            div { class: "nd-dropdown-input-wrapper",
                input {
                    class: "nd-dropdown-input nd-surface-inset nd-shadow-inset",
                    value: "{input_value}",
                    placeholder: props.placeholder,
                    readonly: if !props.searchable { "true" } else { "false" },
                    autocomplete: "off",
                    onclick: move |_| {
                        if !props.searchable {
                            is_open.set(!is_open());
                        }
                    },
                    onblur: move |_| {
                        is_open.set(false);
                    },
                    oninput: move |evt| {
                        *input_value.write() = evt.value().clone();
                        if props.searchable {
                            is_open.set(true);
                        }
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            is_open.set(false);
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
                    let filtered: Vec<_> = props.options
                        .iter()
                        .filter(|o| {
                            if o.disabled {
                                return false;
                            }
                            if props.searchable && !search.is_empty() {
                                return o.label.to_lowercase().contains(&search.to_lowercase());
                            }
                            true
                        })
                        .cloned()
                        .collect();

                    rsx! {
                        div {
                            class: "nd-dropdown-menu",

                            for option in filtered {
                                button {
                                    class: "nd-dropdown-item",
                                    class: if search == option.value { "nd-dropdown-item-selected" } else { "" },
                                    onclick: {
                                        let mut input_value = input_value.clone();
                                        move |_| {
                                            *input_value.write() = option.label.clone();
                                            props.onchange.clone().call(option.value.clone());
                                            *is_open.write() = false;
                                        }
                                    },
                                    "{option.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
