//! Form Control Components
//!
//! Provides Toggle (switch), Checkbox, Radio (radio button)

use crate::theme::use_theme_config;
use dioxus::prelude::*;

// ==================== Toggle 切换开关 ====================

/// Toggle
#[derive(Props, PartialEq, Clone)]
pub struct ToggleProps {
    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,
    /// Change event
    pub on_change: EventHandler<bool>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Toggle component
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    // 原始 HTML：选中时轨道是紫色渐变，未选中时是 neu-inset
    let track_style = if props.checked {
        "background: linear-gradient(145deg, #7c3aed, #6d28d9); box-shadow: 4px 4px 8px rgba(0,0,0,0.2);"
    } else {
        &format!(
            "background: linear-gradient(145deg, {}, {}); \
             box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
            theme.bg_secondary, theme.bg_primary, theme.shadow_dark, theme.shadow_light
        )
    };

    // 拇指位置动画
    let thumb_pos = if props.checked {
        "calc(100% - 28px)"
    } else {
        "4px"
    };

    let thumb_bg = format!(
        "background: linear-gradient(145deg, {}, {}); \
         box-shadow: 2px 2px 4px {}, -2px -2px 4px {};",
        theme.bg_primary, theme.bg_secondary, theme.shadow_dark, theme.shadow_light
    );

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        "cursor: pointer;"
    };

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            "aria-checked": if props.checked { "true" } else { "false" },
            disabled: if props.disabled { "true" } else { "false" },
            class: "neu-toggle {class}",
            style: format!(
                "width: 56px; height: 32px; border-radius: 16px; position: relative; \
                 transition: all 0.3s ease; border: none; outline: none; \
                 {} {}",
                track_style, disabled_style
            ),
            onclick: move |_| {
                if !props.disabled {
                    props.on_change.call(!props.checked);
                }
            },
            div {
                style: format!(
                    "position: absolute; top: 4px; width: 24px; height: 24px; \
                     border-radius: 12px; transition: left 0.3s ease; \
                     {} left: {};",
                    thumb_bg, thumb_pos
                ),
            }
        }
    }
}

// ==================== Checkbox 复选框 ====================

/// Checkbox
#[derive(Props, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,
    /// Change event
    pub on_change: EventHandler<bool>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Checkbox component
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    // 背景样式
    let bg_style = if props.checked {
        "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
         box-shadow: 4px 4px 8px rgba(0,0,0,0.2);"
    } else {
        &format!(
            "background: linear-gradient(145deg, {}, {}); \
             box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
            theme.bg_secondary, theme.bg_primary, theme.shadow_dark, theme.shadow_light
        )
    };

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        "cursor: pointer;"
    };

    // 对勾透明度动画
    let check_opacity = if props.checked { "1" } else { "0" };

    rsx! {
        button {
            r#type: "button",
            role: "checkbox",
            "aria-checked": if props.checked { "true" } else { "false" },
            disabled: if props.disabled { "true" } else { "false" },
            class: "neu-checkbox {class}",
            style: format!(
                "width: 24px; height: 24px; border-radius: 8px; display: flex; \
                 align-items: center; justify-content: center; transition: all 0.2s ease; \
                 border: none; outline: none; {} {}",
                bg_style, disabled_style
            ),
            onclick: move |_| {
                if !props.disabled {
                    props.on_change.call(!props.checked);
                }
            },
            // 对勾图标 - 使用 opacity 过渡动画
            span {
                style: format!(
                    "color: white; font-size: 12px; font-weight: bold; \
                     opacity: {}; transition: opacity 0.2s ease;",
                    check_opacity
                ),
                "✓"
            }
        }
    }
}

/// ==================== Radio 单选按钮 ====================

/// Radio
#[derive(Props, PartialEq, Clone)]
pub struct RadioProps {
    /// Whether it is checked
    #[props(default)]
    pub checked: bool,
    /// Whether it is disabled
    #[props(default)]
    pub disabled: bool,
    /// Value of this radio button (used for grouping)
    pub value: String,
    /// Change event (passes the value of this radio button)
    pub on_change: EventHandler<String>,
    /// Label text (optional)
    #[props(default)]
    pub label: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Radio component
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    // 始终是凹陷效果
    let bg_style = format!(
        "background: linear-gradient(145deg, {}, {}); \
         box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
        theme.bg_secondary, theme.bg_primary, theme.shadow_dark, theme.shadow_light
    );

    let disabled_style = if props.disabled {
        "opacity: 0.6; cursor: not-allowed;"
    } else {
        "cursor: pointer;"
    };

    // 小圆点透明度动画
    let dot_opacity = if props.checked { "1" } else { "0" };

    // 如果有标签，渲染 Radio + Label（按钮在文字前）
    if let Some(label_text) = props.label {
        return rsx! {
            div {
                class: "neu-radio-with-label {class}",
                style: "display: flex; align-items: center; gap: 12px;",
                button {
                    r#type: "button",
                    role: "radio",
                    "aria-checked": if props.checked { "true" } else { "false" },
                    disabled: if props.disabled { "true" } else { "false" },
                    style: format!(
                        "width: 24px; height: 24px; border-radius: 12px; display: flex; \
                         align-items: center; justify-content: center; transition: all 0.2s ease; \
                         border: none; outline: none; {} {}",
                        bg_style, disabled_style
                    ),
                    onclick: move |_| {
                        if !props.disabled {
                            props.on_change.call(props.value.clone());
                        }
                    },
                    // 小圆点 - 使用 opacity 过渡
                    div {
                        style: format!(
                            "width: 12px; height: 12px; border-radius: 6px; \
                             background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                             opacity: {}; transition: opacity 0.2s ease;",
                            dot_opacity
                        ),
                    }
                }
                label {
                    style: "font-size: 14px; color: inherit; cursor: pointer;",
                    "{label_text}"
                }
            }
        };
    }

    rsx! {
        button {
            r#type: "button",
            role: "radio",
            "aria-checked": if props.checked { "true" } else { "false" },
            disabled: if props.disabled { "true" } else { "false" },
            class: "neu-radio {class}",
            style: format!(
                "width: 24px; height: 24px; border-radius: 12px; display: flex; \
                 align-items: center; justify-content: center; transition: all 0.2s ease; \
                 border: none; outline: none; {} {}",
                bg_style, disabled_style
            ),
            onclick: move |_| {
                if !props.disabled {
                    props.on_change.call(props.value.clone());
                }
            },
            div {
                style: format!(
                    "width: 12px; height: 12px; border-radius: 6px; \
                     background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                     opacity: {}; transition: opacity 0.2s ease;",
                    dot_opacity
                ),
            }
        }
    }
}

// ==================== RadioGroup 单选组 ====================

/// Radio option for RadioGroup
#[derive(Clone, PartialEq)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// RadioGroup
#[derive(Props, PartialEq, Clone)]
pub struct RadioGroupProps {
    /// Current selected value
    pub value: String,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Group label (optional)
    #[props(default)]
    pub legend: Option<String>,
    /// Radio options
    pub options: Vec<RadioOption>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// RadioGroup component
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        fieldset {
            class: "neu-radio-group {class}",
            style: "border: none; margin: 0; padding: 0;",
            if let Some(legend_text) = props.legend {
                legend {
                    style: "font-size: 14px; color: inherit; margin-bottom: 12px;",
                    "{legend_text}"
                }
            }
            div {
                role: "radiogroup",
                class: "neu-radio-options",
                style: "display: flex; flex-direction: column; gap: 12px;",
                for option in &props.options {
                    Radio {
                        checked: props.value == option.value,
                        value: option.value.clone(),
                        disabled: option.disabled,
                        label: Some(option.label.clone()),
                        on_change: props.on_change.clone(),
                    }
                }
            }
        }
    }
}
