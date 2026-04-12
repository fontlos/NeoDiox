use dioxus::prelude::*;

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
    let class = props.class.unwrap_or_default();

    if !props.is_open {
        return rsx! {};
    }

    rsx! {
        // 背景遮罩
        div {
            class: "nd-modal-backdrop",
            onclick: move |_| {
                if props.close_on_backdrop {
                    props.on_close.call(());
                }
            },

            // 模态框内容
            div {
                class: "nd-modal {class}",
                style: format!(
                    "width: 100%; max-width: {};",
                    props.size.max_width()
                ),
                onclick: move |evt| {
                    evt.stop_propagation();
                },

                // 拟物化背景
                div {
                    class: "nd-modal-bg",
                }

                // 模态框内部
                div {
                    class: "nd-modal-content",

                    // 标题栏
                    div {
                        class: "nd-modal-header-wrapper",
                        h2 {
                            class: "nd-modal-title",
                            "{props.title}"
                        }

                        if props.show_close {
                            button {
                                r#type: "button",
                                class: "nd-modal-close",
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
