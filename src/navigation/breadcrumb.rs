use dioxus::prelude::*;

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
