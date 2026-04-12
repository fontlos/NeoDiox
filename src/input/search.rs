use dioxus::prelude::*;

/// SearchInput
#[derive(Props, PartialEq, Clone)]
pub struct SearchInputProps {
    /// Search input value
    pub value: String,
    /// Input event
    pub on_input: EventHandler<String>,
    /// Placeholder text
    #[props(default = "Search...".to_string())]
    pub placeholder: String,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Is Disabled
    #[props(default)]
    pub disabled: bool,
    /// Is Clearable
    #[props(default = true)]
    pub clearable: bool,
    /// Is Auto-focused
    #[props(default)]
    pub autofocus: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Icon
    #[props(default)]
    pub search_icon: Option<String>,
}

/// SearchInput Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     SearchInput {
///         value: query,
///         on_input: move |val| set_query(val),
///         placeholder: "Search countries...".to_string(),
///     }
/// }
/// ```
#[component]
pub fn SearchInput(props: SearchInputProps) -> Element {
    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        ""
    };
    let search_icon = props.search_icon.unwrap_or_else(|| "🔍".to_string());
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-search-input {class}",

            if let Some(label_text) = props.label {
                label {
                    class: "nd-label nd-search-label",
                    "{label_text}"
                }
            }

            div { class: "nd-search-input-wrapper",
                // 搜索图标
                span {
                    class: "nd-search-icon",
                    "{search_icon}"
                }

                input {
                    r#type: "text",
                    value: "{props.value}",
                    placeholder: props.placeholder,
                    disabled: if props.disabled { "true" } else { "false" },
                    autofocus: if props.autofocus { "true" } else { "false" },
                    class: "nd-input nd-input-bg",
                    style: format!(
                        "padding: 12px 40px; border-radius: 12px; font-size: 14px; \
                         {}",
                        disabled_style
                    ),
                    oninput: move |evt| {
                        props.on_input.call(evt.value().clone());
                    },
                }

                // 清除按钮
                if props.clearable && !props.value.is_empty() && !props.disabled {
                    button {
                        r#type: "button",
                        class: "nd-search-clear",
                        onclick: move |_| {
                            props.on_input.call(String::new());
                        },
                        "✕"
                    }
                }
            }
        }
    }
}
