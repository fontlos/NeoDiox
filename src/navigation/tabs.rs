use dioxus::prelude::*;

/// Tab
#[derive(Props, PartialEq, Clone)]
pub struct TabProps {
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
    /// Inline style
    #[props(default)]
    pub style: Option<String>,
    /// Whether this tab is active
    pub is_active: bool,

    /// Click Event
    pub onclick: EventHandler<MouseEvent>,

    /// Child Elements
    pub children: Element,
}

/// Tab Component
///
/// # Example: With Router
///
/// ```rust,ignore
/// let navigator = use_navigator();
/// let route = use_route::<Route>();
///
/// rsx! {
///     Tabs {
///         Tab {
///            is_active: route == Route::Home,
///            on_click: move |_| navigator.push(Route::Home),
///            "Home"
///         },
///         Tab {
///           is_active: route == Route::Settings,
///           on_click: move |_| navigator.push(Route::Settings),
///           "Settings"
///         },
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
///         Tab {
///            is_active: active_tab() == "home",
///            on_click: move |_| active_tab.set("home".to_string()),
///            "Home"
///         },
///         Tab {
///           is_active: active_tab() == "settings",
///           on_click: move |_| active_tab.set("settings".to_string()),
///           "Settings"
///         },
///     }
///     match active_tab() {
///         "home" => rsx! { HomeContent {} },
///         "settings" => rsx! { SettingsContent {} },
///     }
/// }
/// ```
#[component]
pub fn Tab(props: TabProps) -> Element {
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();
    let active = if props.is_active { "nd-tab-active" } else { "" };
    rsx! {
        button {
            r#type: "button",
            role: "tab",
            "aria-selected": props.is_active,
            tabindex: if props.is_active { 0 } else { -1 },
            class: "nd-tab {active} {class}",
            style,
            onclick: move |e| {
                props.onclick.call(e);
            },

            {props.children}
        }
    }
}

/// Tabs
#[derive(Props, PartialEq, Clone)]
pub struct TabsProps {
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
    /// Inline style
    #[props(default)]
    pub style: Option<String>,

    /// Children Tab components
    pub children: Element,
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
///         Tab {
///            is_active: route == Route::Home,
///            on_click: move |_| navigator.push(Route::Home),
///            "Home"
///         },
///         Tab {
///           is_active: route == Route::Settings,
///           on_click: move |_| navigator.push(Route::Settings),
///           "Settings"
///         },
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
///         Tab {
///            is_active: active_tab() == "home",
///            on_click: move |_| active_tab.set("home".to_string()),
///            "Home"
///         },
///         Tab {
///           is_active: active_tab() == "settings",
///           on_click: move |_| active_tab.set("settings".to_string()),
///           "Settings"
///         },
///     }
///     match active_tab() {
///         "home" => rsx! { HomeContent {} },
///         "settings" => rsx! { SettingsContent {} },
///     }
/// }
/// ```
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    rsx! {
        nav {
            class: "nd-tabs {class}",
            style,
            role: "tablist",
            "aria-label": "Tabs",
            {props.children}
        }
    }
}
