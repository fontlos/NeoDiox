use dioxus::prelude::*;

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
            class: "nd-toast-container",
            style: format!(
                "position: fixed; z-index: 9999; max-width: 400px; \
                 pointer-events: none; {}",
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
            class: "nd-toast",
            role: "alert",
            style: format!(
                "background: linear-gradient(145deg, {}, {}); \
                 box-shadow: 4px 4px 8px rgba(0, 0, 0, 0.2); \
                 pointer-events: auto;",
                color_start, color_end
            ),

            // 图标
            span {
                class: "nd-alert-icon",
                "{props.toast.toast_type.icon()}"
            }

            // 内容
            div { class: "nd-toast-content",
                p {
                    class: "nd-toast-title",
                    "{props.toast.title}"
                }
                p {
                    class: "nd-toast-message",
                    "{props.toast.message}"
                }
            }

            // 关闭按钮
            button {
                r#type: "button",
                class: "nd-toast-close",
                onclick: move |_| {
                    props.on_dismiss.call(props.toast.id.clone());
                },
                "✕"
            }
        }
    }
}
