mod advanced;
mod basics;
mod data;
mod feedback;
mod forms;

use dioxus::prelude::*;
use neo_diox::prelude::*;

#[component]
pub fn AppInner() -> Element {
    use_init_theme(ThemeVariant::Light);
    let theme = use_theme();
    let mut toggle = use_toggle_theme();
    let dark = theme.read().is_dark();
    let class = theme.read().theme();

    rsx! {
        div { class: "app {class} nd-text",
            // 主题切换按钮
            div { class: "theme-button",
                Button {
                    width: "56px",
                    height: "56px",
                    padding: "0",
                    onclick: move |_| {
                        toggle();
                    },
                    // iconify 的图标是通过 JS 替换的原标签实现的
                    // 这将使 Dioxus 失去对标签的追踪, 所以只能在上层容器做切换
                    div { style: if dark { "display: none;" },
                        Icon {
                            class: "iconify".to_string(),
                            size: 24,
                            "data-icon": "lucide:sun",
                            "data-width": 24,
                        }
                    }
                    div { style: if !dark { "display: none;" },
                        Icon {
                            class: "iconify".to_string(),
                            size: 24,
                            "data-icon": "lucide:moon",
                            "data-width": 24,
                        }
                    }
                }
            }
            Header {}
            Main {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header {
            NeuRaisedSm { class: "live", border_radius: 9999,
                div { class: "circle" }
                span { "Live Components" }
            }
            h1 { class: "title", "NeoDiox UI" }
            p { class: "description", "A Neuromorphic component library built for Dioxus 0.7" }
        }
    }
}

#[component]
fn Main() -> Element {
    let mut active_tab = use_signal(|| "basics".to_string());
    let toasts = use_signal(Vec::<ToastMessage>::new);

    let dismiss_toast = {
        let mut toasts = toasts.clone();
        move |id: String| {
            let mut toasts_guard = toasts.write();
            if let Some(toast) = toasts_guard.iter_mut().find(|t| t.id == id) {
                toast.is_exiting = true;
            }
            drop(toasts_guard);
            let mut toasts = toasts.clone();
            spawn(async move {
                gloo_timers::future::sleep(std::time::Duration::from_millis(300)).await;
                toasts.write().retain(|t| t.id != id);
            });
        }
    };

    rsx! {
        main {
            Tabs { class: "nav-tabs",
                Tab {
                    is_active: active_tab() == "basics",
                    onclick: move |_| active_tab.set("basics".to_string()),
                    "Basics"
                }
                Tab {
                    is_active: active_tab() == "forms",
                    onclick: move |_| active_tab.set("forms".to_string()),
                    "Forms"
                }
                Tab {
                    is_active: active_tab() == "feedback",
                    onclick: move |_| active_tab.set("feedback".to_string()),
                    "Feedback"
                }
                Tab {
                    is_active: active_tab() == "data",
                    onclick: move |_| active_tab.set("data".to_string()),
                    "Data"
                }
                Tab {
                    is_active: active_tab() == "advanced",
                    onclick: move |_| active_tab.set("advanced".to_string()),
                    "Advanced"
                }
            }

            match active_tab().as_str() {
                "basics" => rsx! {
                    basics::Basics {}
                },
                "forms" => rsx! {
                    forms::Forms {}
                },
                "feedback" => rsx! {
                    feedback::Feedback { toasts, on_dismiss: dismiss_toast }
                },
                "data" => rsx! {
                    data::Data {}
                },
                "advanced" => rsx! {
                    advanced::Advanced {}
                },
                _ => rsx! {
                    basics::Basics {}
                },
            }
        }

        // Toast 容器
        ToastContainer { toasts: toasts(), top_offset: 80, on_dismiss: dismiss_toast }
    }
}
