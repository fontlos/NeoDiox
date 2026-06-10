use dioxus::prelude::*;

use crate::icon;

/// StarRating
#[derive(Props, PartialEq, Clone)]
pub struct StarRatingProps {
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,

    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Maximum number of stars
    #[props(default = 5)]
    pub max_stars: u8,
    /// Current rating value
    #[props(default)]
    pub value: u8,

    /// Change event
    pub onchange: EventHandler<u8>,
}

/// StarRating Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     StarRating {
///         value: rating,
///         onchange: move |val| set_rating(val),
///     }
/// }
/// ```
#[component]
pub fn StarRating(props: StarRatingProps) -> Element {
    let class = props.class.unwrap_or_default();

    let mut hover_value = use_signal(|| 0u8);
    let display_value = if hover_value() > 0 {
        hover_value()
    } else {
        props.value
    };

    rsx! {
        div {
            class: "nd-star-rating {class}",
            role: "radiogroup",
            for i in 1..=props.max_stars {
                button {
                    class: "nd-star-rating-button",
                    role: "radio",
                    disabled: props.disabled,
                    onclick: move |_| props.onchange.call(i),
                    onmouseenter: move |_| hover_value.set(i),
                    onmouseleave: move |_| hover_value.set(0),

                    icon::Icon {
                        class: "nd-star-rating-star",
                        size: 28,
                        "data-activated": i <= display_value,
                        icon::Star { }
                    }
                }
            }
        }
    }
}
