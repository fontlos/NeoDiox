//! Advanced Components
//!
//! ContextMenu、TreeView、FileUpload、FeatureCard

use crate::surfaces::{NeuFlat, NeuRaised};
use crate::theme::use_theme_config;
use dioxus::prelude::*;

// ==================== ContextMenu 右键菜单 ====================

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
pub struct ContextMenuProps {
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
///     ContextMenu {
///         items: vec![
///             MenuItem { id: "copy".to_string(), label: "Copy".to_string(), icon: Some("📋".to_string()), disabled: false, is_separator: false },
///             MenuItem { id: "paste".to_string(), label: "Paste".to_string(), icon: Some("📌".to_string()), disabled: false, is_separator: false },
///         ],
///         x: mouse_x,
///         y: mouse_y,
///         is_visible: show_context_menu,
///         on_select: move |id| handle_menu_select(id),
///         on_close: move |_| set_show_context_menu(false),
///     }
/// }
/// ```
#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
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
            style: "position: fixed; inset: 0; z-index: 999;",
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
            class: "neu-context-menu",
            style: format!(
                "position: fixed; left: {adjusted_x}px; top: {adjusted_y}px; \
                 z-index: 1000; min-width: 160px; padding: 8px 0;",
            ),

            for item in items {
                if item.is_separator {
                    div {
                        style: "height: 1px; margin: 4px 0; background: rgba(128, 128, 128, 0.2);",
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
                                style: "font-size: 14px; opacity: 0.7;",
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

fn window_width() -> f64 {
    // TODO: 这应该从浏览器 API 获取
    1920.0
}

fn window_height() -> f64 {
    // TODO: 这应该从浏览器 API 获取
    1080.0
}

// ==================== TreeView 树形视图 ====================

/// Tree node
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub children: Option<Vec<TreeNode>>,
    pub is_leaf: bool,
}

/// Tree View
#[derive(Props, PartialEq, Clone)]
pub struct TreeViewProps {
    /// Tree Node List
    pub nodes: Vec<TreeNode>,
    /// Expanded Node IDs
    #[props(default)]
    pub expanded_nodes: Vec<String>,
    /// Expand/Collapse Event
    pub on_toggle: EventHandler<String>,
    /// Node Selection Event
    pub on_select: EventHandler<String>,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Tree View Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     TreeView {
///         nodes: vec![
///             TreeNode {
///                 id: "src".to_string(),
///                 label: "src".to_string(),
///                 icon: Some("📁".to_string()),
///                 children: Some(vec![
///                     TreeNode { id: "lib.rs".to_string(), label: "lib.rs".to_string(), icon: Some("📄".to_string()), children: None, is_leaf: true },
///                 ]),
///                 is_leaf: false,
///             },
///         ],
///         expanded_nodes: expanded_nodes,
///         on_toggle: move |id| toggle_node(id),
///         on_select: move |id| select_node(id),
///     }
/// }
/// ```
#[component]
pub fn TreeView(props: TreeViewProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        nav {
            class: "neu-tree-view {class}",
            "aria-label": "Tree navigation",
            role: "tree",
            style: "display: flex; flex-direction: column; gap: 4px;",

            for node in &props.nodes {
                TreeNodeItem {
                    node: node.clone(),
                    expanded_nodes: props.expanded_nodes.clone(),
                    on_toggle: props.on_toggle.clone(),
                    on_select: props.on_select.clone(),
                    depth: 0,
                }
            }
        }
    }
}

/// Tree Node Item
#[component]
fn TreeNodeItem(
    node: TreeNode,
    expanded_nodes: Vec<String>,
    on_toggle: EventHandler<String>,
    on_select: EventHandler<String>,
    depth: usize,
) -> Element {
    let is_expanded = expanded_nodes.contains(&node.id);
    let padding_left = depth * 20;

    if node.is_leaf {
        return rsx! {
            div {
                role: "treeitem",
                class: "neu-tree-leaf",
                style: format!("padding-left: {padding_left}px;"),

                div {
                    style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; \
                            border-radius: 8px; cursor: pointer; font-size: 14px; color: inherit; \
                            transition: background 0.15s ease;",
                    onclick: move |_| {
                        on_select.call(node.id.clone());
                    },

                    // 占位符 (对齐展开/收起箭头)
                    span {
                        style: "width: 14px; flex-shrink: 0;",
                    }

                    if let Some(icon) = &node.icon {
                        span {
                            style: "font-size: 16px;",
                            "{icon}"
                        }
                    }

                    "{node.label}"
                }
            }
        };
    }

    rsx! {
        div {
            role: "treeitem",
            class: "neu-tree-item",
            "aria-expanded": if is_expanded { "true" } else { "false" },
            style: format!("padding-left: {padding_left}px;"),

            // 展开/收起按钮
            button {
                r#type: "button",
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; \
                        border-radius: 8px; cursor: pointer; font-size: 14px; color: inherit; \
                        background: none; border: none; width: 100%; text-align: left; \
                        transition: background 0.15s ease;",
                onclick: move |_| {
                    on_toggle.call(node.id.clone());
                },

                // 展开箭头
                span {
                    style: format!(
                        "width: 14px; flex-shrink: 0; font-size: 12px; transition: transform 0.2s ease; \
                         transform: {};",
                        if is_expanded { "rotate(90deg)" } else { "rotate(0deg)" }
                    ),
                    "▶"
                }

                if let Some(icon) = &node.icon {
                    span {
                        style: "font-size: 16px;",
                        "{icon}"
                    }
                } else if is_expanded {
                    span {
                        style: "font-size: 16px;",
                        "📂"
                    }
                } else {
                    span {
                        style: "font-size: 16px;",
                        "📁"
                    }
                }

                "{node.label}"
            }

            // 子节点
            if is_expanded {
                if let Some(children) = &node.children {
                    div {
                        role: "group",
                        style: "margin-top: 4px;",
                        for child in children {
                            TreeNodeItem {
                                node: child.clone(),
                                expanded_nodes: expanded_nodes.clone(),
                                on_toggle: on_toggle.clone(),
                                on_select: on_select.clone(),
                                depth: depth + 1,
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==================== FileUpload 文件上传 ====================

/// File Information
#[derive(Clone, Debug, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub file_type: String,
}

/// File Upload
#[derive(Props, PartialEq, Clone)]
pub struct FileUploadProps {
    /// List of Uploaded Files
    pub files: Vec<FileInfo>,
    /// Add File Event
    pub on_add: EventHandler<Vec<FileInfo>>,
    /// Remove File Event
    pub on_remove: EventHandler<usize>,
    /// Accepted File Types
    #[props(default)]
    pub accept: Option<String>,
    /// Whether to support multiple file selection
    #[props(default = true)]
    pub multiple: bool,
    /// Maximum File Size (bytes)
    #[props(default)]
    pub max_size: Option<u64>,
    /// Drag and Drop Area Text
    #[props(default = "Drop files here or click to upload".to_string())]
    pub drop_text: String,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// File Upload Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     FileUpload {
///         files: uploaded_files,
///         on_add: move |new_files| add_files(new_files),
///         on_remove: move |idx| remove_file(idx),
///         accept: Some("image/*,.pdf".to_string()),
///         max_size: Some(10 * 1024 * 1024), // 10MB
///     }
/// }
/// ```
#[component]
pub fn FileUpload(props: FileUploadProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-file-upload {class}",
            style: "display: flex; flex-direction: column; gap: 16px;",

            // 拖拽区域
            div {
                style: format!(
                    "border: 2px dashed rgba(128, 128, 128, 0.3); border-radius: 16px; \
                     padding: 32px; text-align: center; cursor: pointer; \
                     transition: all 0.3s ease; \
                     background: linear-gradient(145deg, {}, {});",
                    theme.bg_primary, theme.bg_secondary
                ),
                onclick: move |_| {
                    // 在实际应用中，这里应该触发文件选择对话框
                },

                div {
                    style: "display: flex; flex-direction: column; align-items: center; gap: 16px;",

                    // 上传图标
                    div {
                        style: format!(
                            "width: 64px; height: 64px; border-radius: 16px; \
                             display: flex; align-items: center; justify-content: center; \
                             background: linear-gradient(145deg, {}, {}); \
                             box-shadow: 4px 4px 8px {}, -4px -4px 8px {};",
                            theme.bg_primary, theme.bg_secondary,
                            theme.shadow_dark, theme.shadow_light
                        ),
                        span {
                            style: "font-size: 32px; color: #7c3aed;",
                            "📤"
                        }
                    }

                    p {
                        style: "font-size: 14px; font-weight: 500; color: inherit; margin: 0;",
                        "{props.drop_text}"
                    }

                    p {
                        style: "font-size: 12px; color: inherit; opacity: 0.6; margin: 0;",
                        "PNG, JPG, PDF up to 10MB"
                    }
                }
            }

            // 文件列表
            if !props.files.is_empty() {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    for (index, file) in props.files.iter().enumerate() {
                        div {
                            style: format!(
                                "display: flex; align-items: center; gap: 12px; \
                                 padding: 12px; border-radius: 12px; \
                                 background: linear-gradient(145deg, {}, {}); \
                                 box-shadow: 3px 3px 6px {}, -3px -3px 6px {};",
                                theme.bg_primary, theme.bg_secondary,
                                theme.shadow_dark, theme.shadow_light
                            ),

                            // 文件图标
                            span {
                                style: "font-size: 20px; flex-shrink: 0;",
                                "📄"
                            }

                            // 文件信息
                            div {
                                style: "flex: 1; min-width: 0;",
                                p {
                                    style: "font-size: 14px; font-weight: 500; color: inherit; \
                                            margin: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                                    "{file.name}"
                                }
                                p {
                                    style: "font-size: 12px; color: inherit; opacity: 0.6; margin: 2px 0 0 0;",
                                    "{format_file_size(file.size)}"
                                }
                            }

                            // 移除按钮
                            button {
                                r#type: "button",
                                style: "background: none; border: none; cursor: pointer; \
                                        color: inherit; opacity: 0.6; padding: 8px; \
                                        font-size: 16px; border-radius: 8px; \
                                        transition: all 0.15s ease;",
                                onclick: move |_| {
                                    props.on_remove.call(index);
                                },
                                "✕"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_file_size(size: u64) -> String {
    if size < 1024 {
        format!("{size} B")
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    }
}

// ==================== FeatureCard 特性卡片 ====================

/// Feature Card
#[derive(Props, PartialEq, Clone)]
pub struct FeatureCardProps {
    /// Card Title
    pub title: String,
    /// Card Icon
    pub icon: String,
    /// Icon Background Color Gradient
    #[props(default = ("#7c3aed".to_string(), "#6d28d9".to_string()))]
    pub icon_gradient: (String, String),
    /// Whether hover effect
    #[props(default = true)]
    pub hoverable: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Card Content
    pub children: Element,
}

/// Feature Card Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     FeatureCard {
///         title: "Fast".to_string(),
///         icon: "⚡".to_string(),
///         NeuFlat {
///             border_radius: 8,
///             "Lightning fast performance"
///         }
///     }
/// }
/// ```
#[component]
pub fn FeatureCard(props: FeatureCardProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        NeuFlat {
            border_radius: 16,
            class: "neu-feature-card {class}",
            style: if props.hoverable {
                "padding: 16px; text-align: center; transition: all 0.3s ease;"
            } else {
                "padding: 16px; text-align: center;"
            },

            // 图标容器
            div {
                style: format!(
                    "width: 48px; height: 48px; border-radius: 12px; margin: 0 auto 12px; \
                     display: flex; align-items: center; justify-content: center; \
                     background: linear-gradient(145deg, {}, {}); \
                     box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.15);",
                    props.icon_gradient.0, props.icon_gradient.1
                ),
                span {
                    style: "font-size: 24px; color: white;",
                    "{props.icon}"
                }
            }

            // 标题
            p {
                style: "font-size: 14px; font-weight: 500; color: inherit; margin: 0;",
                "{props.title}"
            }

            // 内容
            {props.children}
        }
    }
}
