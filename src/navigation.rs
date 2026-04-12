//! Navigation Components
//!
//! Provides Tabs, Breadcrumbs, Stepper

use dioxus::prelude::*;

use crate::theme::use_theme;

// ==================== Tabs 标签页 ====================

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

// ==================== Breadcrumbs 面包屑 ====================

/// Breadcrumb Item
#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub is_current: bool,
}

/// Breadcrumbs
#[derive(Props, PartialEq, Clone)]
pub struct BreadcrumbsProps {
    /// Breadcrumb items
    pub items: Vec<BreadcrumbItem>,
    /// Separator character (default: "/")
    #[props(default = "/".to_string())]
    pub separator: String,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Breadcrumbs Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Breadcrumbs {
///         items: vec![
///             BreadcrumbItem { label: "Home".to_string(), href: Some("/".to_string()), is_current: false },
///             BreadcrumbItem { label: "Products".to_string(), href: Some("/products".to_string()), is_current: false },
///             BreadcrumbItem { label: "Details".to_string(), href: None, is_current: true },
///         ],
///     }
/// }
/// ```
#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        nav {
            class: "nd-breadcrumbs {class}",
            "aria-label": "Breadcrumbs",
            for (index, item) in props.items.iter().enumerate() {
                div { class: "nd-breadcrumb-item",
                    if item.is_current {
                        span { class: "nd-breadcrumb-current", "{item.label}" }
                    } else if let Some(href) = &item.href {
                        a {
                            class: "nd-breadcrumb-link",
                            href: "{href}",
                            "{item.label}"
                        }
                        span { class: "nd-breadcrumb-separator", "{props.separator}" }
                    } else {
                        span { "{item.label}" }
                        if index < props.items.len() - 1 || !props.items.iter().skip(index + 1).any(|i| i.is_current) {
                            span { class: "nd-breadcrumb-separator", "{props.separator}" }
                        }
                    }
                }
            }
        }
    }
}

// ==================== Stepper 步骤条 ====================

/// Step Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    #[default]
    Pending,
    Active,
    Completed,
    Current,
}

impl StepStatus {
    fn style(&self) -> String {
        match self {
            Self::Completed | Self::Current => format!(
                "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                 color: white; box-shadow: 4px 4px 8px var(--nd-shadow-dark), -4px -4px 8px var(--nd-shadow-light);",
            ),
            Self::Active => format!(
                "background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); \
                 box-shadow: inset 3px 3px 6px var(--nd-shadow-dark), inset -3px -3px 6px var(--nd-shadow-light);",
            ),
            Self::Pending => format!(
                "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                 box-shadow: inset 3px 3px 6px var(--nd-shadow-dark), inset -3px -3px 6px var(--nd-shadow-light);",
            ),
        }
    }

    fn icon(&self) -> Option<&'static str> {
        match self {
            Self::Completed | Self::Current => Some("✓"),
            _ => None,
        }
    }
}

/// Step Item
#[derive(Clone, Debug, PartialEq)]
pub struct StepItem {
    pub label: String,
    pub status: StepStatus,
    pub icon: Option<String>,
}

/// Stepper Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl StepperDirection {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Horizontal => "nd-stepper",
            Self::Vertical => "nd-stepper nd-stepper-vertical",
        }
    }
}

/// Stepper
#[derive(Props, PartialEq, Clone)]
pub struct StepperProps {
    /// Steps
    pub steps: Vec<StepItem>,
    /// Currently active step index
    #[props(default)]
    pub current_step: Option<usize>,
    /// Direction
    #[props(default)]
    pub direction: StepperDirection,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Stepper Component
#[component]
pub fn Stepper(props: StepperProps) -> Element {
    let class = props.class.unwrap_or_default();
    let stepper_class = props.direction.css_class();

    rsx! {
        div {
            class: "{stepper_class} {class}",
            role: "navigation",
            for (index, step) in props.steps.iter().enumerate() {
                div {
                    class: if props.direction == StepperDirection::Vertical {
                        "nd-step nd-step-vertical"
                    } else {
                        "nd-step"
                    },

                    // 步骤指示器
                    div {
                        class: "nd-step-indicator",
                        style: step.status.style(),
                        if let Some(icon) = step.icon.as_ref() {
                            "{icon}"
                        } else if let Some(icon) = step.status.icon() {
                            "{icon}"
                        } else {
                            "{index + 1}"
                        }
                    }

                    // 步骤内容
                    div { class: "nd-step-content",
                        p { class: "nd-step-title", "{step.label}" }
                    }
                }
            }
        }
    }
}
