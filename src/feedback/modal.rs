use dioxus::prelude::*;

use crate::icon;

/// Modal Props
#[derive(Props, PartialEq, Clone)]
pub struct ModalProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Whether the modal is open
    pub is_open: bool,
    /// Modal title
    pub title: String,
    /// Modal size
    #[props(default = "500px".to_string())]
    pub max_width: String,
    /// Whether clicking the backdrop closes the modal
    #[props(default = true)]
    pub close_on_backdrop: bool,
    /// Close event handler
    pub on_close: EventHandler<()>,

    /// Modal body content
    pub children: Element,
}

/// Modal component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Modal {
///         is_open: is_modal_open,
///         title: "Modal Title",
///         on_close: move |_| set_is_modal_open(false),
///         p { "Modal content goes here" }
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
        div {
            class: "nd-modal-backdrop",
            role: "presentation",
            onclick: move |_| {
                if props.close_on_backdrop {
                    props.on_close.call(());
                }
            },

            div {
                class: "nd-modal {class}",
                role: "dialog",
                "aria-modal": "true",
                style: format!("width: 90%; max-width: {};", props.max_width),
                onclick: move |evt| {
                    evt.stop_propagation();
                },

                div { class: "nd-modal-header",

                    h2 { class: "nd-modal-title", "{props.title}" }
                    button {
                        r#type: "button",
                        class: "nd-modal-close",
                        "aria-label": "Close modal",
                        onclick: move |_| {
                            props.on_close.call(());
                        },
                        icon::Icon { size: 16, icon::Close {} }
                    }
                }

                div { class: "nd-modal-body", {props.children} }
            }
        }
    }
}
