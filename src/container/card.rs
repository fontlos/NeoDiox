use crate::surfaces::NeuFlat;
use dioxus::prelude::*;

/// Card
#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
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

/// Card Component
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
pub fn Card(props: CardProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        NeuFlat {
            border_radius: 16,
            class: "nd-feature-card {class}",
            style: if props.hoverable {
                "transition: all 0.3s ease;"
            } else {
                ""
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
                class: "nd-feature-icon-container",
                span {
                    "{props.icon}"
                }
            }

            // 标题
            p {
                class: "nd-feature-title",
                "{props.title}"
            }

            // 内容
            {props.children}
        }
    }
}
