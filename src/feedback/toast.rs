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
    /// Whether the toast is in its exit animation phase.
    /// Set this to `true` before removing the toast to play the exit animation.
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
    /// Top offset in pixels (default: 80px to clear the theme toggle button)
    #[props(default = 80)]
    pub top_offset: u32,
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
    pub fn style(&self, top_offset: u32) -> String {
        match self {
            Self::TopRight => format!("top: {}px; right: 16px; left: auto;", top_offset),
            Self::TopLeft => format!("top: {}px; left: 16px; right: auto;", top_offset),
            Self::BottomRight => "bottom: 16px; right: 16px; left: auto; top: auto;".to_string(),
            Self::BottomLeft => "bottom: 16px; left: 16px; right: auto; top: auto;".to_string(),
            Self::TopCenter => format!(
                "top: {}px; left: 50%; transform: translateX(-50%); right: auto;",
                top_offset
            ),
            Self::BottomCenter => {
                "bottom: 16px; left: 50%; transform: translateX(-50%); right: auto; top: auto;"
                    .to_string()
            }
        }
    }
}

/// Toast Container component - purely presentational.
///
/// The caller manages all timing:
/// - Auto-dismiss: spawn a timer, then call `on_dismiss(id)`
/// - Close button: click calls `on_dismiss(id)`
///
/// `on_dismiss` should:
/// 1. Find the toast and set `is_exiting = true` (triggers exit animation)
/// 2. Wait 300ms for animation to complete
/// 3. Remove the toast from the list
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
        div {
            class: "nd-toast-container",
            style: format!(
                "position: fixed; z-index: 9999; max-width: 400px; \
                 pointer-events: none; {position_style}",
            ),
            for toast in &props.toasts {
                div {
                    key: "{toast.id}",
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

/// Toast Item component - purely presentational.
/// Animation is driven entirely by `toast.is_exiting` prop.
#[component]
fn ToastItem(props: ToastItemProps) -> Element {
    let (color_start, color_end) = props.toast.toast_type.gradient();
    let animation = if props.toast.is_exiting {
        "toast-out 0.3s ease-in forwards"
    } else {
        "toast-in 0.3s ease-out"
    };

    rsx! {
        div {
            class: "nd-toast",
            role: "alert",
            style: format!(
                "background: linear-gradient(145deg, {}, {}); \
                 box-shadow: 4px 4px 8px rgba(0, 0, 0, 0.2); \
                 pointer-events: auto; \
                 animation: {animation};",
                color_start, color_end
            ),

            // Icon
            span {
                class: "nd-alert-icon",
                "{props.toast.toast_type.icon()}"
            }

            // Content
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

            // Close button
            button {
                r#type: "button",
                class: "nd-toast-close",
                onclick: move |_| props.on_dismiss.call(props.toast.id.clone()),
                "✕"
            }
        }
    }
}
