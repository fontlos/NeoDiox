use dioxus::prelude::*;

/// Modal Size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ModalSize {
    pub fn max_width(&self) -> &'static str {
        match self {
            Self::Small => "400px",
            Self::Medium => "500px",
            Self::Large => "700px",
        }
    }
}

/// Modal Props
#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    /// Whether the modal is open
    pub is_open: bool,
    /// Close event handler
    pub on_close: EventHandler<()>,
    /// Modal title
    pub title: String,
    /// Modal body content
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
    /// Optional footer content
    #[props(default)]
    pub footer: Option<Element>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Modal component - neuromorphic dialog with backdrop
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Modal {
///         is_open: is_modal_open,
///         on_close: move |_| set_is_modal_open(false),
///         title: "Modal Title",
///         p { "Modal content goes here" }
///         footer: rsx! {
///             Button { variant: ButtonVariant::Neuromorphic, onclick: move |_| set_is_modal_open(false), "Cancel" }
///             Button { variant: ButtonVariant::PRIMARY, onclick: move |_| { set_is_modal_open(false); }, "Confirm" }
///         },
///     }
/// }
/// ```
#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.is_open {
        return rsx! {};
    }

    let class = props.class.unwrap_or_default();

    rsx! {
        // Backdrop
        div {
            class: "nd-modal-backdrop",
            role: "presentation",
            onclick: move |_| {
                if props.close_on_backdrop {
                    props.on_close.call(());
                }
            },

            // Modal dialog
            div {
                class: "nd-modal {class}",
                role: "dialog",
                "aria-modal": "true",
                style: format!("width: 90%; max-width: {};", props.size.max_width()),
                onclick: move |evt| {
                    evt.stop_propagation();
                },

                // Header
                div {
                    class: "nd-modal-header",

                    h2 {
                        class: "nd-modal-title",
                        "{props.title}"
                    }

                    if props.show_close {
                        button {
                            r#type: "button",
                            class: "nd-modal-close",
                            "aria-label": "Close modal",
                            onclick: move |_| {
                                props.on_close.call(());
                            },
                            "✕"
                        }
                    }
                }

                // Body
                div {
                    class: "nd-modal-body",
                    {props.children}
                }

                // Footer (optional)
                if let Some(footer) = &props.footer {
                    div {
                        class: "nd-modal-footer",
                        {footer.clone()}
                    }
                }
            }
        }
    }
}
