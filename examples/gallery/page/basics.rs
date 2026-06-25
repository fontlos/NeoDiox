use dioxus::prelude::*;
use neo_diox::prelude::*;

#[component]
pub fn Basics() -> Element {
    rsx! {
        div { class: "page",
            // Buttons
            ButtonPanel {}
            // Toggles
            TogglePanel {}
            // Badges
            BadgePanel {}
            // tips
            TipsPanel {}
            // Accordion - 跨两列
            AccordionPanel {}
        }
    }
}

#[component]
fn ButtonPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:mouse-pointer-click",
                    "data-width": 20,
                }
                "Buttons"
            }
            div { style: "display: flex; flex-direction: column; gap: 16px;",
                Button {
                    variant: ButtonVariant::Neuromorphic,
                    onclick: move |_| {},
                    "Default"
                }
                Button { variant: ButtonVariant::PRIMARY, onclick: move |_| {}, "Primary" }
                Button { variant: ButtonVariant::SUCCESS, onclick: move |_| {}, "Success" }
            }
        }
    }
}

#[component]
fn TogglePanel() -> Element {
    let mut notifications = use_signal(|| false);
    let mut terms = use_signal(|| false);
    let mut plan = use_signal(|| "free".to_string());

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:toggle-left",
                    "data-width": 20,
                }
                "Toggles & Checks"
            }
            div { style: "display: flex; flex-direction: column; gap: 24px;",
                div { style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 14px; opacity: 0.7;", "Enable notifications" }
                    Toggle {
                        checked: notifications(),
                        onchange: move |val| notifications.set(val),
                    }
                }
                div { style: "display: flex; align-items: center; gap: 12px;",
                    Checkbox { checked: terms(), onchange: move |val| terms.set(val) }
                    span { style: "font-size: 14px; opacity: 0.7;", "Accept terms" }
                }
                fieldset { style: "border: none; margin: 0; padding: 0;",
                    legend { style: "font-size: 14px; margin-bottom: 12px;", "Choose plan:" }
                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        label { class: "radio-option",
                            Radio {
                                checked: plan() == "free",
                                name: "plan",
                                value: "free",
                                onchange: move |val| plan.set(val),
                            }
                            span { class: "radio-label", "Free" }
                        }
                        label { class: "radio-option",
                            Radio {
                                checked: plan() == "pro",
                                name: "plan",
                                value: "pro",
                                onchange: move |val| plan.set(val),
                            }
                            span { class: "radio-label", "Pro" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BadgePanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:badge",
                    "data-width": 20,
                }
                "Badges"
            }
            div { style: "display: flex; flex-wrap: wrap; gap: 12px;",
                Badge { variant: ButtonVariant::Neuromorphic, "Default" }
                Badge { variant: ButtonVariant::PRIMARY, "Primary" }
                Badge { variant: ButtonVariant::SUCCESS, "Success" }
                Badge { variant: ButtonVariant::WARNING, "Warning" }
                Badge { variant: ButtonVariant::DANGER, "Error" }
                Badge { variant: ButtonVariant::INFO, "Info" }
            }
        }
    }
}

#[component]
fn TipsPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:message-square",
                    "data-width": 20,
                }
                "tips"
            }
            div { style: "display: flex; flex-wrap: wrap; gap: 16px;",
                Tip { content: "Home", position: TipPosition::Top,
                    Button { width: "44px", height: "44px", padding: "0",
                        Icon {
                            class: "iconify",
                            size: 20,
                            "data-icon": "lucide:home",
                        }
                    }
                }
                Tip { content: "Settings", position: TipPosition::Top,
                    Button { width: "44px", height: "44px", padding: "0",
                        Icon {
                            class: "iconify",
                            size: 20,
                            "data-icon": "lucide:settings",
                        }
                    }
                }
                Tip { content: "Notifications", position: TipPosition::Top,
                    Button { width: "44px", height: "44px", padding: "0",
                        Icon {
                            class: "iconify",
                            size: 20,
                            "data-icon": "lucide:bell",
                        }
                    }
                }
                Tip { content: "Profile", position: TipPosition::Top,
                    Button { width: "44px", height: "44px", padding: "0",
                        Icon {
                            class: "iconify",
                            size: 20,
                            "data-icon": "lucide:user",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel big-panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:chevrons-down",
                    "data-width": 20,
                }
                "Accordion"
            }
            div { style: "display: flex; flex-direction: column; gap: 16px;",
                Accordion {
                    title: "What is neuromorphic design?",
                    content: "Neuromorphic design uses soft shadows and gradients to create a soft, extruded plastic look. It mimics real physical objects with subtle depth.",
                    expanded: true,
                }
                Accordion {
                    title: "Is it accessible?",
                    content: "Yes! This implementation includes full keyboard navigation, ARIA attributes, and screen reader support.",
                }
                Accordion {
                    title: "Can I customize it?",
                    content: "Absolutely! All components are built with customization in mind. Colors, sizes, and animations can be easily modified.",
                }
            }
        }
    }
}
