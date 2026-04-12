use crate::surfaces::NeuRaised;
use dioxus::prelude::*;

/// Menu item
#[derive(Clone, Debug, PartialEq)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub disabled: bool,
    pub is_separator: bool,
}

/// Menu
#[derive(Props, PartialEq, Clone)]
pub struct MenuProps {
    /// Menu item list
    pub items: Vec<MenuItem>,
    /// Menu Position X
    pub x: f64,
    /// Menu Position Y
    pub y: f64,
    /// Whether to show
    pub is_visible: bool,
    /// Select menu item event
    pub on_select: EventHandler<String>,
    /// Close menu event
    pub on_close: EventHandler<()>,
}

/// Right-click Menu Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Menu {
///         items: vec![
///             MenuItem { id: "copy".to_string(), label: "Copy".to_string(), icon: Some("📋".to_string()), disabled: false, is_separator: false },
///             MenuItem { id: "paste".to_string(), label: "Paste".to_string(), icon: Some("📌".to_string()), disabled: false, is_separator: false },
///         ],
///         x: mouse_x,
///         y: mouse_y,
///         is_visible: show_context_menu,
///         on_select: move |id| handle_menu_select(id),
///         on_close: move |_| set_show_menu(false),
///     }
/// }
/// ```
#[component]
pub fn Menu(props: MenuProps) -> Element {
    // let theme = use_theme_config();
    let items = props.items.clone();
    let on_select = props.on_select.clone();
    let on_close = props.on_close.clone();

    if !props.is_visible {
        return rsx! {};
    }

    // 计算菜单是否超出视口
    let menu_width = 160.0;
    let menu_height = items.len() as f64 * 36.0;

    let adjusted_x = if props.x + menu_width > window_width() {
        window_width() - menu_width - 10.0
    } else {
        props.x
    };

    let adjusted_y = if props.y + menu_height > window_height() {
        window_height() - menu_height - 10.0
    } else {
        props.y
    };

    rsx! {
        // 背景遮罩, 用于关闭
        div {
            class: "nd-context-menu-backdrop",
            onclick: {
                let on_close = on_close.clone();
                move |_| {
                    on_close.call(());
                }
            },
        }

        // 菜单
        NeuRaised {
            border_radius: 12,
            class: "nd-context-menu",
            style: format!(
                "position: fixed; left: {adjusted_x}px; top: {adjusted_y}px; \
                 z-index: 1000; min-width: 160px; padding: 8px 0;",
            ),

            for item in items {
                if item.is_separator {
                    div {
                        class: "nd-context-menu-separator",
                        role: "separator",
                    }
                } else {
                    button {
                        r#type: "button",
                        role: "menuitem",
                        disabled: if item.disabled { "true" } else { "false" },
                        style: format!(
                            "width: 100%; padding: 10px 16px; display: flex; align-items: center; \
                             gap: 10px; font-size: 14px; color: inherit; background: none; \
                             border: none; cursor: {}; text-align: left; transition: background 0.15s ease; \
                             {}",
                            if item.disabled { "default" } else { "pointer" },
                            if item.disabled { "opacity: 0.5;" } else { "" }
                        ),
                        onclick: {
                            let item_id = item.id.clone();
                            let on_select = on_select.clone();
                            let on_close = on_close.clone();
                            let disabled = item.disabled;
                            move |_| {
                                if !disabled {
                                    on_select.call(item_id.clone());
                                    on_close.call(());
                                }
                            }
                        },
                        onmouseenter: move |_| {
                            // hover 效果
                        },

                        if let Some(icon) = &item.icon {
                            span {
                                class: "nd-context-menu-item-icon",
                                "{icon}"
                            }
                        }

                        "{item.label}"
                    }
                }
            }
        }
    }
}

// TODO: 这应该从浏览器 API 获取
fn window_width() -> f64 {
    1920.0
}

fn window_height() -> f64 {
    1080.0
}
