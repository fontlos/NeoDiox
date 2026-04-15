use dioxus::prelude::*;

/// Tree node type
#[derive(Debug, Clone, PartialEq)]
pub enum TreeNodeKind {
    /// Folder node (expandable)
    Folder {
        /// Whether the folder is expanded
        is_expanded: bool,
        /// Child nodes
        children: Vec<TreeNode>,
    },
    /// File/leaf node (not expandable)
    File,
}

/// Tree node
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    /// Iconify icon name for the item (e.g. "lucide:folder-open")
    pub icon: Option<String>,
    /// Node type
    pub kind: TreeNodeKind,
}

/// Tree View Props
#[derive(Props, PartialEq, Clone)]
pub struct TreeViewProps {
    /// Root tree nodes
    pub nodes: Vec<TreeNode>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Tree View Component
///
/// A neuromorphic file-tree navigation with expandable folders.
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
///                 icon: Some("lucide:folder-open".to_string()),
///                 kind: TreeNodeKind::Folder {
///                     is_expanded: true,
///                     children: vec![
///                         TreeNode {
///                             id: "lib.rs".to_string(),
///                             label: "lib.rs".to_string(),
///                             icon: Some("lucide:file-code".to_string()),
///                             kind: TreeNodeKind::File,
///                         },
///                     ],
///                 },
///             },
///         ],
///     }
/// }
/// ```
#[component]
pub fn TreeView(props: TreeViewProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        nav {
            class: "nd-tree-view {class}",
            role: "tree",
            "aria-label": "File tree navigation",

            for node in &props.nodes {
                TreeNodeItem { node: node.clone() }
            }
        }
    }
}

/// Internal tree node item component with self-managed expand state
#[component]
fn TreeNodeItem(node: TreeNode) -> Element {
    match node.kind {
        TreeNodeKind::Folder {
            is_expanded,
            children,
        } => {
            let mut expanded = use_signal(|| is_expanded);

            rsx! {
                li {
                    role: "treeitem",
                    "aria-expanded": if *expanded.read() { "true" } else { "false" },
                    class: "nd-tree-item",

                    button {
                        r#type: "button",
                        class: "nd-tree-toggle",
                        onclick: move |_| {
                            let is_exp = *expanded.peek();
                            *expanded.write() = !is_exp;
                        },

                        // Chevron
                        span {
                            class: "iconify nd-tree-chevron",
                            "data-icon": "lucide:chevron-right",
                            "data-width": 14,
                            class: if *expanded.read() { "nd-tree-chevron-expanded" } else { "" },
                        }

                        // Folder icon
                        if let Some(ref icon) = node.icon {
                            span {
                                class: "iconify nd-tree-icon",
                                "data-icon": "{icon}",
                                "data-width": 18,
                            }
                        }

                        span { "{node.label}" }
                    }

                    // Children
                    if *expanded.read() && !children.is_empty() {
                        ul {
                            role: "group",
                            class: "nd-tree-children",
                            for child in children {
                                TreeNodeItem { node: child.clone() }
                            }
                        }
                    }
                }
            }
        }
        TreeNodeKind::File => {
            rsx! {
                li {
                    role: "treeitem",
                    class: "nd-tree-leaf",

                    div {
                        class: "nd-tree-leaf-content",

                        // Spacer (aligns with chevron)
                        span { class: "nd-tree-leaf-spacer" }

                        // File icon
                        if let Some(ref icon) = node.icon {
                            span {
                                class: "iconify nd-tree-icon",
                                "data-icon": "{icon}",
                                "data-width": 18,
                            }
                        }

                        span { "{node.label}" }
                    }
                }
            }
        }
    }
}
