use dioxus::prelude::*;
use neo_diox::prelude::*;

use std::fmt::Display;

#[component]
pub fn Forms() -> Element {
    rsx! {
        div { class: "page",

            FormPanel {}
            DropdownPanel {}
            DateTimePanel {}
            SliderPanel {}
            DateTimePanel {}
            FilePanel {}
        }
    }
}

#[component]
fn FormPanel() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut submitted = use_signal(|| false);

    let name_error = if name().is_empty() && submitted() {
        Some("Name is required".to_string())
    } else {
        None
    };
    let email_error = if submitted() {
        if email().is_empty() {
            Some("Email is required".to_string())
        } else if !email().contains('@') {
            Some("Please enter a valid email".to_string())
        } else {
            None
        }
    } else if !email().is_empty() && !email().contains('@') {
        Some("Please enter a valid email".to_string())
    } else {
        None
    };

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:text-cursor-input",
                    "data-width": 20,
                }
                "Inputs"
            }
            div { style: "display: flex; flex-direction: column; gap: 20px;",
                div {
                    p { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Name (Required)"
                    }
                    TextInput {
                        error: name_error.is_some(),
                        name: "name",
                        placeholder: "First Name",
                        value: name(),
                        oninput: move |val| {
                            *name.write() = val;
                            *submitted.write() = false;
                        },
                    }
                    if let Some(t) = name_error {
                        p { class: "nd-error", "{t}" }
                    }
                }
                div {
                    p { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Email (Required)"
                    }
                    TextInput {
                        error: email_error.is_some(),
                        name: "email",
                        placeholder: "email@example.com",
                        r#type: "email",
                        value: email(),
                        oninput: move |val| {
                            *email.write() = val;
                            *submitted.write() = false;
                        },
                    }
                    if let Some(t) = email_error {
                        p { class: "nd-error", "{t}" }
                    }
                }
                div {
                    p { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Message"
                    }
                    TextArea {
                        style: "max-height: 200px;",
                        name: "message",
                        value: message(),
                        placeholder: "Your message...",
                        oninput: move |val| *message.write() = val,
                    }
                }
                Button {
                    variant: ButtonVariant::PRIMARY,
                    onclick: move |_| {
                        *submitted.write() = true;
                    },
                    "Submit"
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Country {
    US,
    UK,
    CA,
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::US => write!(f, "United States"),
            Country::UK => write!(f, "United Kingdom"),
            Country::CA => write!(f, "Canada"),
        }
    }
}

#[component]
fn DropdownPanel() -> Element {
    let options = vec![Country::US, Country::UK, Country::CA];
    let mut country = use_signal(|| Option::<Country>::None);
    let mut skills = use_signal(Vec::<&'static str>::new);

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:search",
                    "data-width": 20,
                }
                "Dropdown"
            }
            div { style: "display: flex; flex-direction: column; gap: 20px;",
                // Dropdown
                div {
                    label { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Country"
                    }
                    // 和 TextInput 自动填充冲突
                    Dropdown {
                        options,
                        value: country(),
                        onchange: move |val| *country.write() = Some(val),
                        searchable: false,
                        placeholder: "Search countries...".to_string(),
                    }
                }

                // Multi-Select
                div {
                    label { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Skills"
                    }
                    MultiSelect {
                        options: vec![
                                                    "JavaScript",
                                                    "TypeScript",
                                                    "React",
                                                    "Vue.js",
                                                    "Node.js",
                                                    "Python",
                                                ],
                        values: skills(),
                        onchange: move |vals| *skills.write() = vals,
                        placeholder: "Click to select...".to_string(),
                    }
                }
            }
        }
    }
}

#[component]
fn SliderPanel() -> Element {
    let mut volume = use_signal(|| 50i32);
    let mut rating = use_signal(|| 0u8);

    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:sliders-horizontal",
                    "data-width": 20,
                }
                "Range & Rating"
            }
            div { style: "display: flex; flex-direction: column; gap: 24px;",
                div { style: "display: flex; justify-content: space-between; align-items: center;",
                    span { style: "font-size: 14px; font-weight: 500; color: inherit;",
                        "Volume"
                    }
                    span { style: "font-size: 14px; font-weight: 600; color: #7c3aed;",
                        "{volume}"
                    }
                }
                Slider {
                    value: volume(),
                    onchange: move |val| *volume.write() = val,
                }
                label { style: "font-size: 14px; font-weight: 500; color: inherit;", "Rating" }
                StarRating {
                    value: rating(),
                    onchange: move |val| *rating.write() = val,
                }
            }
        }
    }
}

#[component]
fn DateTimePanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:calendar-clock",
                    "data-width": 20,
                }
                "Date & Time"
            }
            div { style: "display: flex; flex-direction: column; gap: 20px;",
                div {
                    label { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Select Date"
                    }
                    DatePicker { value: None, on_change: move |_| {} }
                }
                div {
                    label { style: "display: block; font-size: 14px; font-weight: 500; margin-bottom: 6px;",
                        "Select Time"
                    }
                    TimePicker { value: None, on_change: move |_| {} }
                }
            }
        }
    }
}

#[component]
fn FilePanel() -> Element {
    let mut files = use_signal(Vec::<FileInfo>::new);

    rsx! {
        NeuRaised { class: "panel big-panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:upload-cloud",
                    "data-width": 20,
                }
                "File Upload"
            }
            FileUpload {
                files: files(),
                on_add: move |new_files| {
                    let mut f = files.write();
                    f.extend(new_files);
                },
                on_remove: move |idx| {
                    files.write().remove(idx);
                },
                accept: Some("image/*,.pdf,.doc,.docx".to_string()),
            }
        }
    }
}
