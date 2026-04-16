use dioxus::prelude::*;

/// Icon
#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    /// Custom class. Can be used for icon library (e.g., "iconify", "fa fa-home")
    #[props(default)]
    pub class: Option<String>,
    /// Inline style
    #[props(default)]
    pub style: Option<String>,
    /// Icon size in pixels (default: 20)
    #[props(default = 20)]
    pub size: u32,
    /// Icon color (CSS color value)
    #[props(default)]
    pub color: Option<String>,

    /// Additional attributes. Can be used for icon library (e.g., "data-icon", "data-width")
    #[props(extends = GlobalAttributes)]
    pub attrs: Vec<Attribute>,

    /// Custom Element variant (SVG, img, etc.)
    #[props(default)]
    pub children: Element,
}

/// Icon Component
///
/// **Note:** Do NOT provide both `children` and icon `class`/`attrs` simultaneously.
/// However, you CAN combine `children` with non-icon `class`/`attrs` (e.g., layout classes, custom data attributes).
///
/// Class variant:
/// ```rust,ignore
/// rsx! {
///     script { src: "https://code.iconify.design/3/3.1.0/iconify.min.js" }
///     Icon {
///         class: "iconify",
///         size: 24,
///         "data-icon": "lucide:sun",
///         "data-width": 24,
///     }
/// }
/// ```
///
/// Element variant (SVG, img, etc.):
/// ```rust,ignore
/// rsx! {
///     Icon {
///         size: 24,
///         svg {
///             width: "24", height: "24", viewBox: "0 0 24 24",
///             path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
///         }
///     }
/// }
/// ```
#[component]
pub fn Icon(props: IconProps) -> Element {
    let class = props.class.unwrap_or_default();

    let style = format!(
        "width:{}px;height:{}px;{}{}",
        props.size,
        props.size,
        props
            .color
            .map(|c| format!("color: {c};"))
            .unwrap_or_default(),
        props.style.unwrap_or_default()
    );

    rsx! {
        span {
            class: "nd-icon {class}",
            style,
            ..props.attrs,
            {props.children}
        }
    }
}
