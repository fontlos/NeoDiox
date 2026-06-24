use dioxus::prelude::*;

use crate::icon;

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

    pub fn icon(&self) -> Element {
        match self {
            Self::Success => rsx! {
                icon::Checked {}
            },
            Self::Error => rsx! {
                icon::Error {}
            },
            Self::Warning => rsx! {
                icon::Warning {}
            },
            Self::Info => rsx! {
                icon::Info {}
            },
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
    /// Whether the toast is in its exit animation phase.
    pub is_exiting: bool,
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
    /// Top offset in pixels
    #[props(default = 80)]
    pub top_offset: u32,
}

/// Toast Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    #[default]
    TopRight,
    TopLeft,
}

impl ToastPosition {
    pub fn style(&self, top_offset: u32) -> String {
        match self {
            Self::TopRight => format!("top: {}px; right: 16px; left: auto;", top_offset),
            Self::TopLeft => format!("top: {}px; left: 16px; right: auto;", top_offset),
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
///         toasts: toasts(),
///         on_dismiss: move |id| {
///             // Mark as exiting
///             if let Some(t) = toasts.write().iter_mut().find(|t| t.id == id) {
///                 t.is_exiting = true;
///             }
///             // Wait for animation then remove
///             spawn(async move {
///                 gloo_timers::future::sleep(Duration::from_millis(300)).await;
///                 toasts.write().retain(|t| t.id != id);
///             });
///         },
///         top_offset: 80,
///     }
/// }
/// ```
#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    if props.toasts.is_empty() {
        return rsx! {};
    }

    let position_style = props.position.style(props.top_offset);

    rsx! {
        div { class: "nd-toast-container", style: "{position_style}",
            for toast in &props.toasts {
                div { key: "{toast.id}",
                    ToastItem {
                        toast: toast.clone(),
                        on_dismiss: props.on_dismiss.clone(),
                    }
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
            class: if props.toast.is_exiting { "nd-toast-exit" } else { "" },
            role: "alert",
            background: "linear-gradient(145deg, {color_start}, {color_end})",

            icon::Icon { size: 24, color: "white", {props.toast.toast_type.icon()} }

            // Content
            div { class: "nd-toast-content",
                p { class: "nd-toast-title", "{props.toast.title}" }
                p { class: "nd-toast-message", "{props.toast.message}" }
            }

            // Close button
            button {
                r#type: "button",
                class: "nd-toast-close",
                onclick: move |_| props.on_dismiss.call(props.toast.id.clone()),

                icon::Icon { size: 24, color: "white", icon::Close {} }
            }
        }
    }
}
