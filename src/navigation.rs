//! Navigation Components
//!
//! Provides Tabs, Breadcrumbs, Stepper

use crate::theme::use_theme_config;
use dioxus::prelude::*;

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
    /// Tab content
    pub children: Element,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Tabs Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Tabs {
///         options: vec![
///             TabOption { id: "tab1".to_string(), label: "Tab 1".to_string(), disabled: false, icon: None },
///             TabOption { id: "tab2".to_string(), label: "Tab 2".to_string(), disabled: false, icon: None },
///         ],
///         active_tab: active_tab,
///         on_change: move |id| set_active_tab(id),
///         // 内容通过子元素传递
///         rsx! { "Content" }
///     }
/// }
/// ```
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();
    let options = props.options.clone();
    let on_change = props.on_change.clone();

    rsx! {
        nav {
            class: "neu-tabs {class}",
            "aria-label": "Tabs",

            // 标签页列表
            div {
                role: "tablist",
                style: format!(
                    "display: flex; gap: 8px; padding: 8px; border-radius: 16px; overflow-x: auto; \
                     background: linear-gradient(145deg, {}, {}); \
                     box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
                    theme.bg_secondary, theme.bg_primary,
                    theme.shadow_dark, theme.shadow_light
                ),
                for option in &options {
                    button {
                        r#type: "button",
                        role: "tab",
                        id: "tab-{option.id}",
                        "aria-selected": if option.id == props.active_tab { "true" } else { "false" },
                        "aria-controls": "panel-{option.id}",
                        "aria-disabled": if option.disabled { "true" } else { "false" },
                        tabindex: if option.id == props.active_tab { 0 } else { -1 },
                        disabled: if option.disabled { "true" } else { "false" },
                        style: format!(
                            "padding: 12px 24px; border-radius: 12px; font-size: 14px; \
                             font-weight: 500; transition: all 0.2s ease; \
                             display: flex; align-items: center; gap: 8px; \
                             white-space: nowrap; border: none; outline: none; \
                             {}",
                            if option.id == props.active_tab {
                                "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                 color: white; \
                                 box-shadow: 4px 4px 8px rgba(0, 0, 0, 0.2);"
                            } else {
                                "color: inherit; cursor: pointer;"
                            }
                        ),
                        onclick: {
                            let option = option.clone();
                            let on_change = on_change.clone();
                            move |_| {
                                if !option.disabled {
                                    on_change.call(option.id.clone());
                                }
                            }
                        },
                        // onkeydown: {
                        //     let option = option.clone();
                        //     let options = options.clone();
                        //     let on_change = on_change.clone();
                        //     move |evt| {
                        //         // 键盘导航
                        //         let idx = options.iter().position(|o| o.id == option.id).unwrap_or(0);
                        //         let new_idx = match evt.key() {
                        //             Key::ArrowRight => (idx + 1) % options.len(),
                        //             Key::ArrowLeft => if idx == 0 { options.len() - 1 } else { idx - 1 },
                        //             Key::Home => 0,
                        //             Key::End => options.len() - 1,
                        //             _ => {
                        //                 return;
                        //             }
                        //         };
                        //         evt.prevent_default();
                        //         let new_option = &options[new_idx];
                        //         if !new_option.disabled {
                        //             on_change.call(new_option.id.clone());
                        //         }
                        //     }
                        // },

                        if let Some(icon) = &option.icon {
                            span {
                                style: "font-size: 16px;",
                                "{icon}"
                            }
                        }

                        "{option.label}"
                    }
                }
            }

            // 标签页内容容器
            div {
                role: "tabpanel",
                id: "panel-{props.active_tab}",
                "aria-labelledby": "tab-{props.active_tab}",
                tabindex: 0,
                style: "margin-top: 24px;",
                {props.children}
            }
        }
    }
}

// ==================== Breadcrumbs 面包屑导航 ====================

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
    /// Separator icon
    #[props(default = "›".to_string())]
    pub separator: String,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Breadcrumbs component
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
            class: "neu-breadcrumbs {class}",
            "aria-label": "Breadcrumb",

            ol {
                style: "display: flex; align-items: center; gap: 8px; list-style: none; margin: 0; padding: 0; flex-wrap: wrap;",
                for (index, item) in props.items.iter().enumerate() {
                    li {
                        style: "display: flex; align-items: center; gap: 8px;",

                        if item.is_current {
                            // 当前页（不可点击）
                            span {
                                style: "font-size: 14px; font-weight: 500; color: inherit;",
                                "aria-current": "page",
                                "{item.label}"
                            }
                        } else if let Some(href) = &item.href {
                            // 可点击链接
                            a {
                                href: "{href}",
                                style: "font-size: 14px; color: inherit; opacity: 0.7; \
                                        text-decoration: none; transition: opacity 0.15s ease;",
                                onmouseenter: move |_| {
                                    // hover 效果通过 CSS 处理
                                },
                                "{item.label}"
                            }
                        } else {
                            span {
                                style: "font-size: 14px; color: inherit; opacity: 0.7;",
                                "{item.label}"
                            }
                        }

                        // 分隔符（不是最后一项）
                        if index < props.items.len() - 1 {
                            span {
                                style: "font-size: 16px; opacity: 0.5; user-select: none;",
                                "aria-hidden": "true",
                                "{props.separator}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==================== Stepper 步骤条 ====================

/// Step status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    #[default]
    Pending,
    Current,
    Completed,
}

/// Step item
#[derive(Clone, Debug, PartialEq)]
pub struct StepItem {
    pub label: String,
    pub status: StepStatus,
    pub icon: Option<String>,
}

/// Stepper
#[derive(Props, PartialEq, Clone)]
pub struct StepperProps {
    /// Step items
    pub steps: Vec<StepItem>,
    /// Current active step index
    #[props(default)]
    pub current_step: usize,
    /// Step click event (only for non-pending steps)
    #[props(default)]
    pub on_step_click: Option<EventHandler<usize>>,
    /// Stepper direction (horizontal or vertical)
    #[props(default)]
    pub direction: StepperDirection,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Stepper direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperDirection {
    #[default]
    Horizontal,
    Vertical,
}

/// Stepper component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Stepper {
///         steps: vec![
///             StepItem { label: "Account".to_string(), status: StepStatus::Completed, icon: None },
///             StepItem { label: "Details".to_string(), status: StepStatus::Current, icon: None },
///             StepItem { label: "Confirm".to_string(), status: StepStatus::Pending, icon: None },
///         ],
///         current_step: 1,
///         on_step_click: Some(move |idx| set_current_step(idx)),
///     }
/// }
/// ```
#[component]
pub fn Stepper(props: StepperProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.clone().unwrap_or_default();

    if props.direction == StepperDirection::Vertical {
        return rsx! {
            StepperVertical {
                steps: props.steps,
                current_step: props.current_step,
                on_step_click: props.on_step_click,
                class: props.class.clone(),
            }
        };
    }

    rsx! {
        div {
            class: "neu-stepper {class}",
            style: "display: flex; align-items: center; justify-content: space-between;",
            role: "list",
            "aria-label": "Progress steps",

            for (index, step) in props.steps.iter().enumerate() {
                // 步骤项
                div {
                    role: "listitem",
                    style: "display: flex; align-items: center; gap: 12px;",

                    // 步骤圆圈
                    if props.on_step_click.is_some() && step.status != StepStatus::Pending {
                        button {
                            r#type: "button",
                            style: format!(
                                "width: 40px; height: 40px; border-radius: 20px; \
                                 display: flex; align-items: center; justify-content: center; \
                                 font-size: 14px; font-weight: 500; border: none; cursor: pointer; \
                                 transition: all 0.2s ease; \
                                 {}",
                                match step.status {
                                    StepStatus::Completed => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                              color: white; \
                                                              box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                    StepStatus::Current => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                            color: white; \
                                                            box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                    StepStatus::Pending => format!(
                                        "background: linear-gradient(145deg, {}, {}); \
                                         color: inherit; \
                                         box-shadow: 3px 3px 6px {}, -3px -3px 6px {};",
                                        theme.bg_primary, theme.bg_secondary,
                                        theme.shadow_dark, theme.shadow_light
                                    ),
                                }
                            ),
                            onclick: move |_| {
                                if let Some(handler) = props.on_step_click {
                                    handler.call(index);
                                }
                            },

                            if let Some(icon) = &step.icon {
                                span { "{icon}" }
                            } else if step.status == StepStatus::Completed {
                                span { "✓" }
                            } else {
                                "{index + 1}"
                            }
                        }
                    } else {
                        div {
                            style: format!(
                                "width: 40px; height: 40px; border-radius: 20px; \
                                 display: flex; align-items: center; justify-content: center; \
                                 font-size: 14px; font-weight: 500; \
                                 {}",
                                match step.status {
                                    StepStatus::Completed => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                              color: white; \
                                                              box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                    StepStatus::Current => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                            color: white; \
                                                            box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                    StepStatus::Pending => format!(
                                        "background: linear-gradient(145deg, {}, {}); \
                                         color: inherit; opacity: 0.6; \
                                         box-shadow: 3px 3px 6px {}, -3px -3px 6px {};",
                                        theme.bg_primary, theme.bg_secondary,
                                        theme.shadow_dark, theme.shadow_light
                                    ),
                                }
                            ),

                            if let Some(icon) = &step.icon {
                                span { "{icon}" }
                            } else if step.status == StepStatus::Completed {
                                span { "✓" }
                            } else {
                                "{index + 1}"
                            }
                        }
                    }

                    // 步骤标签
                    span {
                        style: format!(
                            "font-size: 14px; font-weight: 500; color: inherit; \
                             {}",
                            match step.status {
                                StepStatus::Pending => "opacity: 0.6;",
                                _ => "",
                            }
                        ),
                        "{step.label}"
                    }
                }

                // 连接线（不是最后一项）
                if index < props.steps.len() - 1 {
                    div {
                        style: format!(
                            "flex: 1; height: 4px; border-radius: 2px; margin: 0 16px; \
                             {}",
                            if step.status == StepStatus::Completed {
                                "background: linear-gradient(90deg, #7c3aed, #a855f7);".to_string()
                            } else {
                                format!(
                                    "background: linear-gradient(145deg, {}, {}); \
                                     box-shadow: inset 2px 2px 4px {}, inset -2px -2px 4px {};",
                                    theme.bg_secondary, theme.bg_primary,
                                    theme.shadow_dark, theme.shadow_light
                                )
                            }
                        ),
                        "aria-hidden": "true",
                    }
                }
            }
        }
    }
}

/// Vertical Stepper component
#[component]
fn StepperVertical(
    steps: Vec<StepItem>,
    current_step: usize,
    #[props(default)] on_step_click: Option<EventHandler<usize>>,
    #[props(default)] class: Option<String>,
) -> Element {
    let theme = use_theme_config();
    let class_str = class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-stepper-vertical {class_str}",
            style: "display: flex; flex-direction: column; gap: 0;",
            role: "list",
            "aria-label": "Progress steps",

            for (index, step) in steps.iter().enumerate() {
                div {
                    role: "listitem",
                    style: "display: flex; gap: 16px;",

                    // 左侧：圆圈 + 连接线
                    div {
                        style: "display: flex; flex-direction: column; align-items: center;",

                        // 步骤圆圈
                        if on_step_click.is_some() && step.status != StepStatus::Pending {
                            button {
                                r#type: "button",
                                style: format!(
                                    "width: 40px; height: 40px; border-radius: 20px; \
                                     display: flex; align-items: center; justify-content: center; \
                                     font-size: 14px; font-weight: 500; border: none; cursor: pointer; \
                                     flex-shrink: 0; transition: all 0.2s ease; \
                                     {}",
                                    match step.status {
                                        StepStatus::Completed => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                                  color: white; \
                                                                  box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                        StepStatus::Current => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                                color: white; \
                                                                box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                        StepStatus::Pending => format!(
                                            "background: linear-gradient(145deg, {}, {}); \
                                             color: inherit; \
                                             box-shadow: 3px 3px 6px {}, -3px -3px 6px {};",
                                            theme.bg_primary, theme.bg_secondary,
                                            theme.shadow_dark, theme.shadow_light
                                        ),
                                    }
                                ),
                                onclick: move |_| {
                                    if let Some(handler) = on_step_click {
                                        handler.call(index);
                                    }
                                },

                                if let Some(icon) = &step.icon {
                                    span { "{icon}" }
                                } else if step.status == StepStatus::Completed {
                                    span { "✓" }
                                } else {
                                    "{index + 1}"
                                }
                            }
                        } else {
                            div {
                                style: format!(
                                    "width: 40px; height: 40px; border-radius: 20px; \
                                     display: flex; align-items: center; justify-content: center; \
                                     font-size: 14px; font-weight: 500; flex-shrink: 0; \
                                     {}",
                                    match step.status {
                                        StepStatus::Completed => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                                  color: white; \
                                                                  box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                        StepStatus::Current => "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                                                color: white; \
                                                                box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.2);".to_string(),
                                        StepStatus::Pending => format!(
                                            "background: linear-gradient(145deg, {}, {}); \
                                             color: inherit; opacity: 0.6; \
                                             box-shadow: 3px 3px 6px {}, -3px -3px 6px {};",
                                            theme.bg_primary, theme.bg_secondary,
                                            theme.shadow_dark, theme.shadow_light
                                        ),
                                    }
                                ),

                                if let Some(icon) = &step.icon {
                                    span { "{icon}" }
                                } else if step.status == StepStatus::Completed {
                                    span { "✓" }
                                } else {
                                    "{index + 1}"
                                }
                            }
                        }

                        // 垂直连接线
                        if index < steps.len() - 1 {
                            div {
                                style: format!(
                                    "width: 4px; flex: 1; min-height: 40px; margin: 8px 0; \
                                     {}",
                                    if step.status == StepStatus::Completed {
                                        "background: linear-gradient(180deg, #7c3aed, #a855f7);".to_string()
                                    } else {
                                        format!(
                                            "background: linear-gradient(145deg, {}, {}); \
                                             box-shadow: inset 2px 2px 4px {}, inset -2px -2px 4px {};",
                                            theme.bg_secondary, theme.bg_primary,
                                            theme.shadow_dark, theme.shadow_light
                                        )
                                    }
                                ),
                                "aria-hidden": "true",
                            }
                        }
                    }

                    // 右侧：标签
                    div {
                        style: "padding-bottom: 24px;",
                        span {
                            style: format!(
                                "font-size: 14px; font-weight: 500; color: inherit; \
                                 {}",
                                match step.status {
                                    StepStatus::Pending => "opacity: 0.6;",
                                    _ => "",
                                }
                            ),
                            "{step.label}"
                        }
                    }
                }
            }
        }
    }
}
