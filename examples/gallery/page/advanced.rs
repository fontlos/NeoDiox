use dioxus::prelude::*;
use neo_diox::prelude::*;

#[component]
pub fn Advanced() -> Element {
    rsx! {
        div { class: "page",
            // Breadcrumbs
            BreadcrumbsPanel {}
            // Stepper
            StepperPanel {}
            // Card
            CardPanel {}
            // Context Menu
            MenuPanel {}
            // Tree View
            TreePanel {}
        }
    }
}

#[component]
fn BreadcrumbsPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:chevrons-right",
                    "data-width": 20,
                }
                "Breadcrumbs"
            }
            Breadcrumbs {
                items: vec![
                    BreadcrumbItem {
                        label: "Home".to_string(),
                        href: Some("#".to_string()),
                        is_current: false,
                    },
                    BreadcrumbItem {
                        label: "Products".to_string(),
                        href: Some("#".to_string()),
                        is_current: false,
                    },
                    BreadcrumbItem {
                        label: "Details".to_string(),
                        href: None,
                        is_current: true,
                    },
                ],
            }
        }
    }
}

#[component]
fn StepperPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel big-panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:git-branch",
                    "data-width": 20,
                }
                "Stepper"
            }
            Stepper {
                steps: vec![
                    StepItem {
                        label: "Account".to_string(),
                        status: StepStatus::Completed,
                    },
                    StepItem {
                        label: "Details".to_string(),
                        status: StepStatus::Current,
                    },
                    StepItem {
                        label: "Confirm".to_string(),
                        status: StepStatus::Pending,
                    },
                ],
                current_step: 1,
            }
        }
    }
}

#[component]
fn CardPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:layout-grid",
                    "data-width": 20,
                }
                "Feature Cards"
            }
            div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",
                // Fast
                Card {
                    div {
                        class: "card-icon",
                        style: "background: linear-gradient(145deg, #7c3aed, #6d28d9);",
                        Icon {
                            class: "iconify",
                            size: 24,
                            "data-icon": "lucide:zap",
                        }
                    }
                    p { class: "card-title", "Fast" }
                }
                // Secure
                Card {
                    div {
                        class: "card-icon",
                        style: "background: linear-gradient(145deg, #22c55e, #16a34a);",
                        Icon {
                            class: "iconify",
                            size: 24,
                            "data-icon": "lucide:shield-check",
                        }
                    }
                    p { class: "card-title", "Secure" }
                }
                // Modern
                Card {
                    div {
                        class: "card-icon",
                        style: "background: linear-gradient(145deg, #eab308, #ca8a04);",
                        Icon {
                            class: "iconify",
                            size: 24,
                            "data-icon": "lucide:sparkles",
                        }
                    }
                    p { class: "card-title", "Modern" }
                }
                // Global
                Card {
                    div {
                        class: "card-icon",
                        style: "background: linear-gradient(145deg, #3b82f6, #2563eb);",
                        Icon {
                            class: "iconify",
                            size: 24,
                            "data-icon": "lucide:globe",
                        }
                    }
                    p { class: "card-title", "Global" }
                }
            }
        }
    }
}

#[component]
fn MenuPanel() -> Element {
    let mut show_context_menu = use_signal(|| false);
    let mut menu_x = use_signal(|| 0.0);
    let mut menu_y = use_signal(|| 0.0);
    let mut selected_action = use_signal(|| "No action yet".to_string());

    let context_items = vec![
        MenuItem {
            id: "copy".to_string(),
            label: "Copy".to_string(),
            icon: Some("lucide:copy".to_string()),
            disabled: false,
            is_separator: false,
            danger: false,
        },
        MenuItem {
            id: "cut".to_string(),
            label: "Cut".to_string(),
            icon: Some("lucide:scissors".to_string()),
            disabled: false,
            is_separator: false,
            danger: false,
        },
        MenuItem {
            id: "paste".to_string(),
            label: "Paste".to_string(),
            icon: Some("lucide:clipboard".to_string()),
            disabled: false,
            is_separator: false,
            danger: false,
        },
        MenuItem {
            id: "sep1".to_string(),
            label: String::new(),
            icon: None,
            disabled: false,
            is_separator: true,
            danger: false,
        },
        MenuItem {
            id: "delete".to_string(),
            label: "Delete".to_string(),
            icon: Some("lucide:trash-2".to_string()),
            disabled: false,
            is_separator: false,
            danger: true,
        },
    ];

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:more-horizontal",
                    "data-width": 20,
                }
                "Context Menu"
            }

            // Trigger area
            div {
                class: "nd-context-trigger",
                oncontextmenu: move |evt| {
                    evt.prevent_default();
                    let data = evt.data();
                    *menu_x.write() = data.page_coordinates().x as f64;
                    *menu_y.write() = data.page_coordinates().y as f64;
                    *show_context_menu.write() = true;
                },
                p { class: "nd-context-trigger-text", "Right-click here" }
                p { class: "nd-context-trigger-hint", "to open context menu" }
            }

            // Last action display
            p { style: "font-size: 12px; margin-top: 16px; opacity: 0.6;",
                "Last action: {selected_action}"
            }

            // Context Menu
            Menu {
                items: context_items.clone(),
                x: *menu_x.read(),
                y: *menu_y.read(),
                is_visible: *show_context_menu.read(),
                on_select: move |id| {
                    *selected_action.write() = format!("Selected: {}", id);
                    *show_context_menu.write() = false;
                },
                on_close: move |_| {
                    *show_context_menu.write() = false;
                },
            }
        }
    }
}

#[component]
fn TreePanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:folder-tree",
                    "data-width": 20,
                }
                "Tree View"
            }
            TreeView {
                nodes: vec![
                    // src folder
                    TreeNode {
                        id: "src".to_string(),
                        label: "src".to_string(),
                        icon: Some("lucide:folder-open".to_string()),
                        kind: TreeNodeKind::Folder {
                            is_expanded: true,
                            children: vec![
                                // components subfolder
                                TreeNode {
                                    id: "components".to_string(),
                                    label: "components".to_string(),
                                    icon: Some("lucide:folder".to_string()),
                                    kind: TreeNodeKind::Folder {
                                        is_expanded: false,
                                        children: vec![
                                            TreeNode {
                                                id: "Button.tsx".to_string(),
                                                label: "Button.tsx".to_string(),
                                                icon: Some("lucide:file-code".to_string()),
                                                kind: TreeNodeKind::File,
                                            },
                                            TreeNode {
                                                id: "Modal.tsx".to_string(),
                                                label: "Modal.tsx".to_string(),
                                                icon: Some("lucide:file-code".to_string()),
                                                kind: TreeNodeKind::File,
                                            },
                                            TreeNode {
                                                id: "Card.tsx".to_string(),
                                                label: "Card.tsx".to_string(),
                                                icon: Some("lucide:file-code".to_string()),
                                                kind: TreeNodeKind::File,
                                            },
                                        ],
                                    },
                                },
                                TreeNode {
                                    id: "App.tsx".to_string(),
                                    label: "App.tsx".to_string(),
                                    icon: Some("lucide:file-code".to_string()),
                                    kind: TreeNodeKind::File,
                                },
                                TreeNode {
                                    id: "index.ts".to_string(),
                                    label: "index.ts".to_string(),
                                    icon: Some("lucide:file".to_string()),
                                    kind: TreeNodeKind::File,
                                },
                            ],
                        },
                    },
                    TreeNode {
                        id: "package.json".to_string(),
                        label: "package.json".to_string(),
                        icon: Some("lucide:file-json".to_string()),
                        kind: TreeNodeKind::File,
                    },
                    TreeNode {
                        id: "README.md".to_string(),
                        label: "README.md".to_string(),
                        icon: Some("lucide:file-text".to_string()),
                        kind: TreeNodeKind::File,
                    },
                ],
            }
        }
    }
}
