//! Feedback Components
//!
//! Provides ProgressBar, Toast, Modal, Alert, Skeleton, Spinner

use crate::theme::use_theme_config;
use dioxus::prelude::*;

// ==================== ProgressBar 进度条 ====================

/// Progress Bar
#[derive(Props, PartialEq, Clone)]
pub struct ProgressBarProps {
    /// Current progress value (0-100)
    #[props(default = 0)]
    pub value: u8,
    /// Whether to show progress text
    #[props(default = true)]
    pub show_label: bool,
    /// Label text (overrides default percentage)
    #[props(default)]
    pub label: Option<String>,
    /// Progress bar color gradient (start color, end color)
    #[props(default)]
    pub color: Option<(String, String)>,
    /// Whether to show shimmer effect
    #[props(default)]
    pub shimmer: bool,
    /// Height of the progress bar in pixels
    #[props(default = 12)]
    pub height: u32,
    /// Custom class name for styling
    #[props(default)]
    pub class: Option<String>,
}

/// Progress Bar component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     ProgressBar {
///         value: progress,
///         show_label: true,
///     }
/// }
/// ```
#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let theme = use_theme_config();
    let label = props.label.unwrap_or_else(|| format!("{}%", props.value));
    let class = props.class.unwrap_or_default();
    let (color_start, color_end) = props
        .color
        .unwrap_or_else(|| ("#7c3aed".to_string(), "#a855f7".to_string()));

    rsx! {
        div {
            class: "neu-progress-bar {class}",
            style: "display: flex; flex-direction: column; gap: 8px;",
            role: "progressbar",
            "aria-valuenow": props.value,
            "aria-valuemin": 0,
            "aria-valuemax": 100,

            if props.show_label {
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    span {
                        style: "font-size: 14px; color: inherit;",
                        {label.clone()}
                    }
                }
            }

            // 进度条轨道
            div {
                style: format!(
                    "height: {}px; border-radius: 6px; overflow: hidden; \
                     background: linear-gradient(145deg, {}, {}); \
                     box-shadow: inset 3px 3px 6px {}, inset -3px -3px 6px {};",
                    props.height,
                    theme.bg_secondary, theme.bg_primary,
                    theme.shadow_dark, theme.shadow_light
                ),

                // 进度填充
                div {
                    style: format!(
                        "height: 100%; width: {}%; border-radius: 6px; \
                         background: linear-gradient(90deg, {}, {}); \
                         transition: width 0.3s ease; position: relative;",
                        props.value, color_start, color_end
                    ),

                    // 闪烁效果
                    if props.shimmer {
                        div {
                            style: "position: absolute; inset: 0; \
                                    background: linear-gradient(90deg, \
                                        transparent, \
                                        rgba(255, 255, 255, 0.4), \
                                        transparent); \
                                    animation: shimmer 1.5s infinite;",
                        }
                    }
                }
            }
        }
    }
}

// ==================== Toast 提示 ====================

/// Toast Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl ToastType {
    pub fn gradient(&self) -> (&'static str, &'static str) {
        match self {
            Self::Success => ("#22c55e", "#16a34a"),
            Self::Error => ("#ef4444", "#dc2626"),
            Self::Warning => ("#eab308", "#ca8a04"),
            Self::Info => ("#3b82f6", "#2563eb"),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Success => "✓",
            Self::Error => "✕",
            Self::Warning => "⚠",
            Self::Info => "ℹ",
        }
    }
}

/// Toast Message
#[derive(Clone, Debug, PartialEq)]
pub struct ToastMessage {
    pub id: String,
    pub toast_type: ToastType,
    pub title: String,
    pub message: String,
}

/// Toast Container
#[derive(Props, PartialEq, Clone)]
pub struct ToastContainerProps {
    /// Toast messages to display
    pub toasts: Vec<ToastMessage>,
    /// Dismiss event handler (receives the ID of the dismissed toast)
    pub on_dismiss: EventHandler<String>,
    /// Toast position on the screen
    #[props(default)]
    pub position: ToastPosition,
}

/// Toast Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
    BottomCenter,
}

impl ToastPosition {
    pub fn style(&self) -> &'static str {
        match self {
            Self::TopRight => "top: 16px; right: 16px; left: auto;",
            Self::TopLeft => "top: 16px; left: 16px; right: auto;",
            Self::BottomRight => "bottom: 16px; right: 16px; left: auto; top: auto;",
            Self::BottomLeft => "bottom: 16px; left: 16px; right: auto; top: auto;",
            Self::TopCenter => "top: 16px; left: 50%; transform: translateX(-50%); right: auto;",
            Self::BottomCenter => {
                "bottom: 16px; left: 50%; transform: translateX(-50%); right: auto; top: auto;"
            }
        }
    }
}

/// Toast Container component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     ToastContainer {
///         toasts: toasts,
///         on_dismiss: move |id| dismiss_toast(id),
///     }
/// }
/// ```
#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    if props.toasts.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            style: format!(
                "position: fixed; z-index: 9999; display: flex; flex-direction: column; \
                 gap: 12px; max-width: 400px; pointer-events: none; {}",
                props.position.style()
            ),
            for toast in &props.toasts {
                ToastItem {
                    toast: toast.clone(),
                    on_dismiss: props.on_dismiss.clone(),
                }
            }
        }
    }
}

/// Toast Item
#[derive(Props, PartialEq, Clone)]
struct ToastItemProps {
    pub toast: ToastMessage,
    pub on_dismiss: EventHandler<String>,
}

/// Toast Item component
#[component]
fn ToastItem(props: ToastItemProps) -> Element {
    let (color_start, color_end) = props.toast.toast_type.gradient();

    rsx! {
        div {
            role: "alert",
            style: format!(
                "display: flex; align-items: flex-start; gap: 12px; \
                 padding: 16px; border-radius: 12px; min-width: 280px; \
                 background: linear-gradient(145deg, {}, {}); \
                 box-shadow: 4px 4px 8px rgba(0, 0, 0, 0.2); \
                 pointer-events: auto;",
                color_start, color_end
            ),

            // 图标
            span {
                style: "color: white; font-size: 20px; margin-top: 2px; flex-shrink: 0;",
                "{props.toast.toast_type.icon()}"
            }

            // 内容
            div {
                style: "flex: 1; min-width: 0;",
                p {
                    style: "font-size: 14px; font-weight: 500; color: white; margin: 0;",
                    "{props.toast.title}"
                }
                p {
                    style: "font-size: 12px; color: rgba(255, 255, 255, 0.8); margin: 4px 0 0 0;",
                    "{props.toast.message}"
                }
            }

            // 关闭按钮
            button {
                r#type: "button",
                style: "background: none; border: none; cursor: pointer; \
                        color: rgba(255, 255, 255, 0.8); padding: 4px; \
                        font-size: 16px; flex-shrink: 0;",
                onclick: move |_| {
                    props.on_dismiss.call(props.toast.id.clone());
                },
                "✕"
            }
        }
    }
}

// ==================== Modal 模态框 ====================

/// Modal
#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    /// Whether to open
    pub is_open: bool,
    /// Close event handler
    pub on_close: EventHandler<()>,
    /// Modal title
    pub title: String,
    /// Modal content
    pub children: Element,
    /// Modal size
    #[props(default)]
    pub size: ModalSize,
    /// Whether to show the close button
    #[props(default = true)]
    pub show_close: bool,
    /// Whether clicking the backdrop closes the modal
    #[props(default = true)]
    pub close_on_backdrop: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Modal size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
    FullScreen,
}

impl ModalSize {
    pub fn max_width(&self) -> &'static str {
        match self {
            Self::Small => "400px",
            Self::Medium => "600px",
            Self::Large => "800px",
            Self::FullScreen => "100%",
        }
    }
}

/// Modal component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Modal {
///         is_open: is_modal_open,
///         on_close: move |_| set_is_modal_open(false),
///         title: "Modal Title".to_string(),
///         NeuFlat {
///             border_radius: 8,
///             "Modal content goes here"
///         }
///     }
/// }
/// ```
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    if !props.is_open {
        return rsx! {};
    }

    rsx! {
        // 背景遮罩
        div {
            style: "position: fixed; inset: 0; z-index: 1000; \
                    background: rgba(0, 0, 0, 0.5); backdrop-filter: blur(4px); \
                    display: flex; align-items: center; justify-content: center; \
                    padding: 16px;",
            onclick: move |_| {
                if props.close_on_backdrop {
                    props.on_close.call(());
                }
            },

            // 模态框内容
            div {
                class: "neu-modal {class}",
                style: format!(
                    "width: 100%; max-width: {}; max-height: 90vh; overflow: auto; \
                     border-radius: 16px; position: relative;",
                    props.size.max_width()
                ),
                onclick: move |evt| {
                    evt.stop_propagation();
                },

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 16px; z-index: -1; \
                         background: linear-gradient(145deg, {}, {}); \
                         box-shadow: 8px 8px 16px {}, -8px -8px 16px {};",
                        theme.bg_primary, theme.bg_secondary,
                        theme.shadow_dark, theme.shadow_light
                    ),
                }

                // 模态框内部
                div {
                    style: "padding: 24px;",

                    // 标题栏
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; \
                                margin-bottom: 16px;",
                        h2 {
                            style: "font-size: 18px; font-weight: 600; color: inherit; margin: 0;",
                            "{props.title}"
                        }

                        if props.show_close {
                            button {
                                r#type: "button",
                                style: "background: none; border: none; cursor: pointer; \
                                        padding: 8px; border-radius: 8px; color: inherit; \
                                        transition: background 0.15s ease;",
                                onclick: move |_| {
                                    props.on_close.call(());
                                },
                                "✕"
                            }
                        }
                    }

                    // 内容
                    {props.children}
                }
            }
        }
    }
}

// ==================== Alert 警告 ====================

/// Alert Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl AlertType {
    pub fn style(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::Success => ("#dcfce7", "#166534", "#bbf7d0", "#15803d"),
            Self::Error => ("#fee2e2", "#991b1b", "#fecaca", "#b91c1c"),
            Self::Warning => ("#fef9c3", "#854d0e", "#fde68a", "#a16207"),
            Self::Info => ("#dbeafe", "#1e40af", "#bfdbfe", "#1d4ed8"),
        }
    }
}

/// Alert
#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    /// Alert type
    #[props(default)]
    pub alert_type: AlertType,
    /// Alert title
    pub title: String,
    /// Alert message
    pub message: String,
    /// Whether to show the close button
    #[props(default)]
    pub dismissible: bool,
    /// Dismiss event
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
    /// Custom icon
    #[props(default)]
    pub icon: Option<String>,
}

/// Alert component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Alert {
///         alert_type: AlertType::Success,
///         title: "Success".to_string(),
///         message: "Your changes have been saved.".to_string(),
///         dismissible: true,
///         on_dismiss: move |_| println!("Alert dismissed"),
///     }
/// }
/// ```
#[component]
pub fn Alert(props: AlertProps) -> Element {
    let (bg_color, text_color, border_color, icon_color) = props.alert_type.style();
    let icon = props.icon.unwrap_or_else(|| match props.alert_type {
        AlertType::Success => "✓".to_string(),
        AlertType::Error => "✕".to_string(),
        AlertType::Warning => "⚠".to_string(),
        AlertType::Info => "ℹ".to_string(),
    });
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            role: "alert",
            class: "neu-alert {class}",
            style: format!(
                "display: flex; align-items: flex-start; gap: 12px; \
                 padding: 16px; border-radius: 12px; \
                 background: {bg_color}; \
                 border-left: 4px solid {border_color};",
            ),

            // 图标
            span {
                style: format!("font-size: 20px; color: {icon_color}; flex-shrink: 0; margin-top: 2px;"),
                "{icon}"
            }

            // 内容
            div {
                style: "flex: 1; min-width: 0;",
                p {
                    style: format!("font-size: 14px; font-weight: 500; color: {text_color}; margin: 0;"),
                    "{props.title}"
                }
                p {
                    style: format!("font-size: 12px; color: {icon_color}; margin: 4px 0 0 0;"),
                    "{props.message}"
                }
            }

            // 关闭按钮
            if props.dismissible {
                button {
                    r#type: "button",
                    style: "background: none; border: none; cursor: pointer; \
                            color: inherit; opacity: 0.6; padding: 4px; font-size: 16px;",
                    onclick: move |_| {
                        if let Some(handler) = props.on_dismiss {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

// ==================== Skeleton 骨架屏 ====================

/// Skeleton
#[derive(Props, PartialEq, Clone)]
pub struct SkeletonProps {
    /// Width (CSS value)
    #[props(default = "100%".to_string())]
    pub width: String,
    /// Height (CSS value)
    #[props(default = "16px".to_string())]
    pub height: String,
    /// Border radius (pixels)
    #[props(default = 8)]
    pub border_radius: u32,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Skeleton component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Skeleton {
///         width: "200px".to_string(),
///         height: "20px".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-skeleton {class}",
            style: format!(
                "width: {}; height: {}; border-radius: {}px; \
                 background: linear-gradient(90deg, \
                     {} 25%, \
                     {} 50%, \
                     {} 75%); \
                 background-size: 200% 100%; \
                 animation: skeleton-loading 1.5s infinite;",
                props.width, props.height, props.border_radius,
                theme.bg_secondary, theme.bg_primary, theme.bg_secondary
            ),
        }
    }
}

/// ==================== Spinner 加载旋转 ====================

/// Spinner Loading
#[derive(Props, PartialEq, Clone)]
pub struct SpinnerProps {
    /// Size (pixels)
    #[props(default = 24)]
    pub size: u32,
    /// Color
    #[props(default = "#7c3aed".to_string())]
    pub color: String,
    /// Border width
    #[props(default = 2)]
    pub border_width: u32,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Spinner component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Spinner {
///         size: 32,
///         color: "#7c3aed".to_string(),
///     }
/// }
/// ```
#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-spinner {class}",
            "aria-label": "Loading",
            style: format!(
                "width: {}px; height: {}px; \
                 border: {}px solid currentColor; \
                 border-top-color: transparent; \
                 border-radius: 50%; \
                 color: {}; \
                 animation: spin 0.6s linear infinite;",
                props.size, props.size, props.border_width, props.color
            ),
        }
    }
}
