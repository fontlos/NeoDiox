use dioxus::prelude::*;

/// Menu item
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    /// Iconify icon name (e.g. "lucide:copy")
    pub icon: Option<String>,
    pub disabled: bool,
    pub is_separator: bool,
    pub danger: bool,
}

/// Context Menu Props
#[derive(Props, PartialEq, Clone)]
pub struct MenuProps {
    /// Menu items
    pub items: Vec<MenuItem>,
    /// Position X (window coordinates)
    pub x: f64,
    /// Position Y (window coordinates)
    pub y: f64,
    /// Whether the menu is visible
    pub is_visible: bool,
    /// Item selected event (returns item id)
    pub on_select: EventHandler<String>,
    /// Close menu event
    pub on_close: EventHandler<()>,
}

/// Context Menu Component
///
/// Renders a neuromorphic right-click context menu.
/// Closes when clicking outside, selecting an item, or pressing Escape.
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Menu {
///         items: vec![
///             MenuItem { id: "copy".to_string(), label: "Copy".to_string(), icon: Some("lucide:copy".to_string()), disabled: false, is_separator: false, danger: false },
///             MenuItem { id: "sep".to_string(), label: String::new(), icon: None, disabled: false, is_separator: true, danger: false },
///             MenuItem { id: "delete".to_string(), label: "Delete".to_string(), icon: Some("lucide:trash-2".to_string()), disabled: false, is_separator: false, danger: true },
///         ],
///         x: mouse_x,
///         y: mouse_y,
///         is_visible: show_context_menu,
///         on_select: move |id| handle_select(id),
///         on_close: move |_| set_show_menu(false),
///     }
/// }
/// ```
#[component]
pub fn Menu(props: MenuProps) -> Element {
    if !props.is_visible {
        return rsx! {};
    }

    let adjusted_x = props.x;
    let adjusted_y = props.y;

    rsx! {
        // Backdrop for click-away dismissal
        div {
            class: "nd-context-menu-backdrop",
            role: "presentation",
            onclick: move |_| {
                props.on_close.call(());
            },
        }

        // Menu panel
        div {
            class: "nd-context-menu",
            role: "menu",
            "aria-label": "Context menu",
            style: format!(
                "position: fixed; left: {adjusted_x}px; top: {adjusted_y}px; \
                 z-index: 1001; min-width: 180px;",
            ),
            onkeydown: move |evt| {
                if evt.key() == Key::Escape {
                    props.on_close.call(());
                }
            },

            for item in &props.items {
                if item.is_separator {
                    div {
                        class: "nd-context-menu-separator",
                        role: "separator",
                    }
                } else {
                    button {
                        r#type: "button",
                        role: "menuitem",
                        class: "nd-context-menu-item",
                        class: if item.danger { "nd-context-menu-item-danger" } else { "" },
                        disabled: item.disabled,
                        onclick: {
                            let item_id = item.id.clone();
                            let on_select = props.on_select.clone();
                            let on_close = props.on_close.clone();
                            let disabled = item.disabled;
                            move |_| {
                                if !disabled {
                                    on_select.call(item_id.clone());
                                    on_close.call(());
                                }
                            }
                        },

                        if let Some(icon) = &item.icon {
                            span {
                                class: "iconify nd-context-menu-icon",
                                "data-icon": "{icon}",
                                "data-width": 14,
                            }
                        }

                        "{item.label}"
                    }
                }
            }
        }
    }
}
