use dioxus::prelude::*;

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
            class: "nd-tree-view {class}",
            "aria-label": "Tree navigation",
            role: "tree",

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
                class: "nd-tree-leaf",
                style: format!("padding-left: {padding_left}px;"),

                div {
                    class: "nd-tree-leaf-content",
                    onclick: move |_| {
                        on_select.call(node.id.clone());
                    },

                    // 占位符 (对齐展开/收起箭头)
                    span {
                        class: "nd-tree-leaf-placeholder",
                    }

                    if let Some(icon) = &node.icon {
                        span {
                            class: "nd-tree-leaf-icon",
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
            class: "nd-tree-item",
            "aria-expanded": if is_expanded { "true" } else { "false" },
            style: format!("padding-left: {padding_left}px;"),

            // 展开/收起按钮
            button {
                r#type: "button",
                class: "nd-tree-item-button",
                onclick: move |_| {
                    on_toggle.call(node.id.clone());
                },

                // 展开箭头
                span {
                    class: "nd-tree-toggle-icon",
                    style: format!("transform: {};", if is_expanded { "rotate(90deg)" } else { "rotate(0deg)" }),
                    "▶"
                }

                if let Some(icon) = &node.icon {
                    span {
                        class: "nd-tree-leaf-icon",
                        "{icon}"
                    }
                } else if is_expanded {
                    span {
                        class: "nd-tree-leaf-icon",
                        "📂"
                    }
                } else {
                    span {
                        class: "nd-tree-leaf-icon",
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
                        class: "nd-tree-children",
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
