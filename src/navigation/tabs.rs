use dioxus::prelude::*;

use crate::theme::use_theme;

/// Tab Option
#[derive(Clone, Debug, PartialEq)]
pub struct TabOption {
    pub id: String,
    pub label: String,
    pub disabled: bool,
    pub icon: Option<String>,
}

/// Tabs
#[derive(Props, PartialEq, Clone)]
pub struct TabsProps {
    /// All tab options
    pub options: Vec<TabOption>,
    /// Currently active tab ID
    pub active_tab: String,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Tabs Component
///
/// # Example: With Router
///
/// ```rust,ignore
/// let navigator = use_navigator();
/// let route = use_route::<Route>();
///
/// rsx! {
///     Tabs {
///         options: vec![
///             TabOption { id: "/".to_string(), label: "Home".to_string(), disabled: false, icon: None },
///             TabOption { id: "/settings".to_string(), label: "Settings".to_string(), disabled: false, icon: None },
///         ],
///         active_tab: route.to_string(),
///         on_change: move |id| navigator.push(id),
///     }
///     Outlet::<Route> {}
/// }
/// ```
///
/// # Example: Manual
///
/// ```rust,ignore
/// let mut active_tab = use_signal(|| "home".to_string());
///
/// rsx! {
///     Tabs {
///         options: my_options,
///         active_tab: active_tab(),
///         on_change: move |id| active_tab.set(id),
///     }
///     match active_tab() {
///         "home" => rsx! { HomeContent {} },
///         "settings" => rsx! { SettingsContent {} },
///     }
/// }
/// ```
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let theme = use_theme();
    let text_color = if theme.is_dark() {
        "#d1d5db"
    } else {
        "#4b5563"
    };
    let class = props.class.unwrap_or_default();

    rsx! {
        nav {
            class: "nd-tabs {class}",
            role: "tablist",
            "aria-label": "Tabs",

            for option in &props.options {
                button {
                    r#type: "button",
                    role: "tab",
                    "aria-selected": if option.id == props.active_tab { "true" } else { "false" },
                    "aria-disabled": if option.disabled { "true" } else { "false" },
                    tabindex: if option.id == props.active_tab { 0 } else { -1 },
                    disabled: if option.disabled { true } else { false },
                    class: "nd-tab",
                    class: if option.id == props.active_tab { "nd-tab-active" } else { "" },
                    style: "color: {text_color};",
                    onclick: {
                        let option = option.clone();
                        let on_change = props.on_change.clone();
                        move |_| {
                            if !option.disabled {
                                on_change.call(option.id.clone());
                            }
                        }
                    },

                    if let Some(icon) = &option.icon {
                        span { class: "nd-tab-icon", "{icon}" }
                    }
                    span { class: "nd-tab-label", "{option.label}" }
                }
            }
        }
    }
}
