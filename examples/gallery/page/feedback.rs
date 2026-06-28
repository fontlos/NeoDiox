use dioxus::prelude::*;
use neo_diox::prelude::*;

use crate::page::TOASTS;

#[component]
pub fn Feedback() -> Element {
    rsx! {
        div { class: "page",

            ProgressPanel {}
            LoadingPanel {}
            ToastPanel {}
            ModalPanel {}
            AlertPanel {}
        }
    }
}

#[component]
fn ProgressPanel() -> Element {
    let mut progress = use_signal(|| 0u8);

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:loader",
                    "data-width": 20,
                }
                "Progress"
            }
            div { style: "display: flex; flex-direction: column; gap: 16px;",
                ProgressBar { value: progress() }
                div { style: "display: flex; gap: 12px;",
                    Button {
                        variant: ButtonVariant::SUCCESS,
                        onclick: move |_| {
                            let current = *progress.read();
                            if current < 100 {
                                *progress.write() = (current + 25).min(100);
                            }
                        },
                        "+25%"
                    }
                    Button {
                        onclick: move |_| {
                            *progress.write() = 0;
                        },
                        "Reset"
                    }
                }
            }
        }
    }
}

#[component]
fn LoadingPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:loader-2",
                    "data-width": 20,
                }
                "Loading"
            }
            div { style: "display: flex; flex-direction: column; gap: 20px;",
                // Spinner
                div { style: "display: flex; align-items: center; gap: 16px;",
                    Spinner {}
                    span { style: "font-size: 14px;", "Spinner" }
                }
                // Dots
                div { style: "display: flex; align-items: center; gap: 16px;",
                    Dots {}
                    span { style: "font-size: 14px;", "Dots" }
                }
                // Skeleton
                div {
                    span { style: "font-size: 14px; font-weight: 500; margin-bottom: 12px; display: block;",
                        "Skeleton"
                    }
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        Skeleton { width: "75%" }
                        Skeleton { width: "100%" }
                        Skeleton { width: "60%" }
                    }
                }
            }
        }
    }
}

#[component]
fn ToastPanel() -> Element {
    // Helper to add a toast
    let add_toast = move |toast_type: ToastType, title: &str, message: &str, duration_ms: u64| {

            let id = TOASTS.write().add(Toast {
                id: 0,
                toast_type,
                title: title.to_string(),
                message: message.to_string(),
                is_exiting: false,
            });

            spawn(async move {
                gloo_timers::future::sleep(std::time::Duration::from_millis(duration_ms)).await;
                TOASTS.write().mark_exiting(id);
                gloo_timers::future::sleep(std::time::Duration::from_millis(300)).await;
                TOASTS.write().remove(id);
            });
        };

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:bell-ring",
                    "data-width": 20,
                }
                "Notifications"
            }
            div { style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;",
                Button {
                    variant: ButtonVariant::SUCCESS,
                    onclick: move |_| add_toast(
                        ToastType::Success,
                        "Success",
                        "Your changes have been saved.",
                        5000,
                    ),
                    "Success"
                }
                Button {
                    variant: ButtonVariant::DANGER,
                    onclick: move |_| add_toast(
                        ToastType::Error,
                        "Error",
                        "There was a problem processing your request.",
                        5000,
                    ),
                    "Error"
                }
                Button {
                    variant: ButtonVariant::WARNING,
                    onclick: move |_| add_toast(
                        ToastType::Warning,
                        "Warning",
                        "Your session will expire in 5 minutes.",
                        5000,
                    ),
                    "Warning"
                }
                Button {
                    variant: ButtonVariant::INFO,
                    onclick: move |_| add_toast(ToastType::Info, "Info", "A new version is available.", 5000),
                    "Info"
                }
            }
        }
    }
}

#[component]
fn ModalPanel() -> Element {
    let mut is_modal_open = use_signal(|| false);

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:layout",
                    "data-width": 20,
                }
                "Modal Dialog"
            }
            Button {
                variant: ButtonVariant::PRIMARY,
                onclick: move |_| *is_modal_open.write() = true,
                width: "100%".to_string(),
                "Open Modal"
            }

            Modal {
                is_open: *is_modal_open.read(),
                on_close: move |_| *is_modal_open.write() = false,
                title: "Modal Title",
                p {
                    class: "nd-text-soft",
                    style: "font-size: 14px; margin: 0;",
                    "This is a beautiful neuromorphic modal dialog with smooth animations and full accessibility support. Press Escape to close, or click outside."
                }

                div { style: "display: flex; gap: 12px;",
                    Button {
                        variant: ButtonVariant::Neuromorphic,
                        onclick: move |_| *is_modal_open.write() = false,
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::PRIMARY,
                        onclick: move |_| *is_modal_open.write() = false,
                        "Confirm"
                    }
                }
            }
        }
    }
}

#[component]
fn AlertPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:alert-triangle",
                    "data-width": 20,
                }
                "Alerts"
            }
            div { style: "display: flex; flex-direction: column; gap: 16px;",
                Alert {
                    alert_type: AlertType::Success,
                    title: "Success".to_string(),
                    message: "Changes saved.".to_string(),
                }
                Alert {
                    alert_type: AlertType::Warning,
                    title: "Warning".to_string(),
                    message: "Session expires soon.".to_string(),
                }
            }
        }
    }
}
