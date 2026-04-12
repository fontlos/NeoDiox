use dioxus::prelude::*;

/// StarRating
#[derive(Props, PartialEq, Clone)]
pub struct StarRatingProps {
    /// Current rating value
    #[props(default)]
    pub value: u8,
    /// Change event
    pub on_change: EventHandler<u8>,
    /// Maximum number of stars
    #[props(default = 5)]
    pub max_stars: u8,
    /// Star size (pixels)
    #[props(default = 28)]
    pub star_size: u32,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Whether to show hover effect
    #[props(default = true)]
    pub hoverable: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// StarRating Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     StarRating {
///         value: rating,
///         on_change: move |val| set_rating(val),
///         label: Some("Rating".to_string()),
///     }
/// }
/// ```
#[component]
pub fn StarRating(props: StarRatingProps) -> Element {
    let mut hover_value = use_signal(|| 0u8);
    // let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    let display_value = if props.hoverable && *hover_value.read() > 0 {
        *hover_value.read()
    } else {
        props.value
    };

    rsx! {
        div {
            class: "nd-star-rating {class}",
            style: "display: flex; flex-direction: column; gap: 8px;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit;",
                    "{label_text}"
                }
            }

            div {
                role: "radiogroup",
                style: "display: flex; gap: 4px;",
                for i in 1..=props.max_stars {
                    button {
                        r#type: "button",
                        role: "radio",
                        "aria-checked": if i == props.value { "true" } else { "false" },
                        "aria-label": format!("{i} star{}", if i > 1 { "s" } else { "" }),
                        disabled: if props.disabled { "true" } else { "false" },
                        style: format!(
                            "padding: 4px; background: none; border: none; cursor: {}; \
                             transition: transform 0.15s ease; border-radius: 4px;",
                            if props.disabled { "default" } else { "pointer" }
                        ),
                        onclick: move |_| {
                            if !props.disabled {
                                props.on_change.call(i);
                            }
                        },
                        onmouseenter: move |_| {
                            if props.hoverable && !props.disabled {
                                *hover_value.write() = i;
                            }
                        },
                        onmouseleave: move |_| {
                            if props.hoverable {
                                *hover_value.write() = 0;
                            }
                        },

                        span {
                            style: format!(
                                "font-size: {}px; color: {}; transition: color 0.15s ease;",
                                props.star_size,
                                if i <= display_value { "#f59e0b" } else { "#d1d5db" }
                            ),
                            "★"
                        }
                    }
                }
            }
        }
    }
}
