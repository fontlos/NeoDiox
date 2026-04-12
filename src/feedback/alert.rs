use dioxus::prelude::*;

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
            class: "nd-alert {class}",
            style: format!(
                "background: {bg_color}; border-left: 4px solid {border_color};",
            ),

            // 图标
            span {
                class: "nd-alert-icon",
                style: format!("color: {icon_color};"),
                "{icon}"
            }

            // 内容
            div { class: "nd-alert-content",
                p {
                    class: "nd-alert-title",
                    style: format!("color: {text_color};"),
                    "{props.title}"
                }
                p {
                    class: "nd-alert-message",
                    style: format!("color: {icon_color};"),
                    "{props.message}"
                }
            }

            // 关闭按钮
            if props.dismissible {
                button {
                    r#type: "button",
                    class: "nd-badge-dismiss",
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
