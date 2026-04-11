//! Selector Component
//!
//! Provides Dropdown, MultiSelect, RangeSlider, StarRating, DatePicker, TimePicker

use crate::surfaces::NeuRaised;
use dioxus::prelude::*;

// ==================== Dropdown 下拉选择 ====================

/// Dropdown Option
#[derive(Clone, Debug, PartialEq)]
pub struct DropdownOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// Dropdown
#[derive(Props, PartialEq, Clone)]
pub struct DropdownProps {
    /// Dropdown Options
    pub options: Vec<DropdownOption>,
    /// Current selected value
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Placeholder text
    #[props(default)]
    pub placeholder: Option<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Whether to enable search
    #[props(default)]
    pub searchable: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Dropdown Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Dropdown {
///         options: vec![
///             DropdownOption { value: "cn".to_string(), label: "China".to_string(), disabled: false },
///             DropdownOption { value: "us".to_string(), label: "United States".to_string(), disabled: false },
///         ],
///         value: selected_country,
///         on_change: move |val| set_selected_country(val),
///         label: Some("Select Country".to_string()),
///     }
/// }
/// ```
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);
    let class = props.class.unwrap_or_default();

    let placeholder = props.placeholder.unwrap_or_else(|| "Select...".to_string());

    // 获取当前选中的标签
    let selected_label = props
        .value
        .as_ref()
        .and_then(|v| props.options.iter().find(|o| &o.value == v))
        .map(|o| o.label.clone())
        .unwrap_or(placeholder.clone());

    // 过滤选项
    let filtered_options: Vec<_> = props
        .options
        .iter()
        .filter(|o| {
            if o.disabled {
                return false;
            }
            if props.searchable && !search_query.read().is_empty() {
                return o
                    .label
                    .to_lowercase()
                    .contains(&search_query.read().to_lowercase());
            }
            true
        })
        .cloned()
        .collect();
    let filtered_is_empty = filtered_options.is_empty();

    rsx! {
        div {
            class: "nd-dropdown {class}",
            style: "position: relative;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit; margin-bottom: 8px; display: block;",
                    "{label_text}"
                }
            }

            // 触发器
            button {
                r#type: "button",
                class: "nd-dropdown-trigger",
                disabled: if props.disabled { "true" } else { "false" },
                style: format!(
                    "width: 100%; padding: 12px 40px 12px 16px; border-radius: 12px; \
                     font-size: 14px; color: {}; background: transparent; \
                     border: none; cursor: pointer; text-align: left; \
                     transition: all 0.2s ease; outline: none; position: relative;",
                    if props.value.is_none() { "inherit" } else { "inherit" }
                ),
                onclick: {
                    let mut is_open = is_open.clone();
                    move |_| {
                        if !props.disabled {
                            let current = *is_open.read();
                            *is_open.write() = !current;
                        }
                    }
                },

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 12px; z-index: -1; \
                         background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
                    ),
                }

                span {
                    style: if props.value.is_none() { "opacity: 0.6;" } else { "" },
                    "{selected_label}"
                }

                // 下拉箭头
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%); transition: transform 0.2s ease;",
                    if *is_open.read() { "▼" } else { "▼" }
                }
            }

            // 下拉列表
            if *is_open.read() {
                NeuRaised {
                    border_radius: 12,
                    class: "nd-dropdown-menu",
                    style: "position: absolute; top: 100%; left: 0; right: 0; \
                            margin-top: 8px; z-index: 50; max-height: 200px; \
                            overflow-y: auto;",

                    // 搜索框（如果启用）
                    if props.searchable {
                        div {
                            style: "padding: 8px; border-bottom: 1px solid rgba(128, 128, 128, 0.2);",
                            input {
                                r#type: "text",
                                value: "{search_query}",
                                placeholder: "Search...",
                                style: "width: 100%; padding: 8px; border: none; \
                                        background: transparent; color: inherit; \
                                        outline: none; font-size: 14px;",
                                oninput: move |evt| {
                                    search_query.write().clone_from(&evt.value());
                                },
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                },
                            }
                        }
                    }

                    // 选项列表
                    div {
                        style: "padding: 4px;",
                        for option in filtered_options {
                            button {
                                r#type: "button",
                                class: "nd-dropdown-item",
                                style: format!(
                                    "width: 100%; padding: 12px 16px; text-align: left; \
                                     background: none; border: none; cursor: pointer; \
                                     font-size: 14px; color: inherit; border-radius: 8px; \
                                     transition: background 0.15s ease;"
                                ),
                                onmouseenter: move |_| {
                                    // hover 效果
                                },
                                onclick: {
                                    let option_value = option.value.clone();
                                    let on_change = props.on_change.clone();
                                    let mut is_open = is_open.clone();
                                    let mut search_query = search_query.clone();
                                    move |_| {
                                        on_change.call(option_value.clone());
                                        *is_open.write() = false;
                                        search_query.write().clear();
                                    }
                                },
                                "{option.label}"
                            }
                        }

                        if filtered_is_empty {
                            div {
                                style: "padding: 12px 16px; text-align: center; \
                                        opacity: 0.6; font-size: 14px;",
                                "No options found"
                            }
                        }
                    }
                }
            }

            // 点击外部关闭
            if *is_open.read() {
                div {
                    style: "position: fixed; inset: 0; z-index: 40;",
                    onclick: {
                        let mut is_open = is_open.clone();
                        let mut search_query = search_query.clone();
                        move |_| {
                            *is_open.write() = false;
                            search_query.write().clear();
                        }
                    },
                }
            }
        }
    }
}

// ==================== MultiSelect 多选 ====================

/// MultiSelect Option
#[derive(Clone, Debug, PartialEq)]
pub struct MultiSelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// MultiSelect
#[derive(Props, PartialEq, Clone)]
pub struct MultiSelectProps {
    /// MultiSelect Options
    pub options: Vec<MultiSelectOption>,
    /// Current selected values
    pub values: Vec<String>,
    /// Change event
    pub on_change: EventHandler<Vec<String>>,
    /// Placeholder text
    #[props(default = "Select...".to_string())]
    pub placeholder: String,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// MultiSelect Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     MultiSelect {
///         options: vec![
///             MultiSelectOption { value: "js".to_string(), label: "JavaScript".to_string(), disabled: false },
///             MultiSelectOption { value: "ts".to_string(), label: "TypeScript".to_string(), disabled: false },
///         ],
///         values: selected_skills,
///         on_change: move |vals| set_selected_skills(vals),
///         label: Some("Skills".to_string()),
///     }
/// }
/// ```
#[component]
pub fn MultiSelect(props: MultiSelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let class = props.class.unwrap_or_default();
    let values = props.values.clone();
    let options = props.options.clone();
    let on_change = props.on_change.clone();

    let toggle_option = move |value: String, values: Vec<String>| {
        let mut values = values;
        if values.contains(&value) {
            values.retain(|v| v != &value);
        } else {
            values.push(value);
        }
        on_change.call(values);
    };

    rsx! {
        div {
            class: "nd-multiselect {class}",
            style: "position: relative;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit; margin-bottom: 8px; display: block;",
                    "{label_text}"
                }
            }

            // 触发器/容器
            button {
                r#type: "button",
                class: "nd-multiselect-trigger",
                disabled: if props.disabled { "true" } else { "false" },
                style: format!(
                    "width: 100%; min-height: 48px; padding: 8px 40px 8px 12px; \
                     border-radius: 12px; display: flex; flex-wrap: wrap; \
                     gap: 8px; align-items: center; cursor: pointer; \
                     transition: all 0.2s ease; outline: none; position: relative;"
                ),
                onclick: {
                    let mut is_open = is_open.clone();
                    move |_| {
                        if !props.disabled {
                            let current = *is_open.read();
                            *is_open.write() = !current;
                        }
                    }
                },

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 12px; z-index: -1; \
                         background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);",
                    ),
                }

                // 已选标签
                for value in &values {
                    if let Some(option) = options.iter().find(|o| &o.value == value) {
                        span {
                            style: "display: inline-flex; align-items: center; gap: 4px; \
                                    padding: 4px 8px; border-radius: 6px; font-size: 12px; \
                                    background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                                    color: white; font-weight: 500;",
                            "{option.label}"
                            button {
                                r#type: "button",
                                style: "background: none; border: none; cursor: pointer; \
                                        color: white; padding: 0; font-size: 12px;",
                                onclick: {
                                    let value = value.clone();
                                    let values = values.clone();
                                    let toggle_option = toggle_option.clone();
                                    move |evt| {
                                        evt.stop_propagation();
                                        toggle_option(value.clone(), values.clone());
                                    }
                                },
                                "✕"
                            }
                        }
                    }
                }

                // 占位符
                if values.is_empty() {
                    span {
                        style: "opacity: 0.6; font-size: 14px;",
                        "{props.placeholder}"
                    }
                }

                // 下拉箭头
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%);",
                    "▼"
                }
            }

            // 下拉列表
            if *is_open.read() {
                NeuRaised {
                    border_radius: 12,
                    class: "nd-multiselect-menu",
                    style: "position: absolute; top: 100%; left: 0; right: 0; \
                            margin-top: 8px; z-index: 50; max-height: 200px; \
                            overflow-y: auto;",
                    div {
                        style: "padding: 4px;",
                        for option in &options {
                            button {
                                r#type: "button",
                                class: "nd-multiselect-item",
                                style: {
                                    let inset_style = format!(
                                        "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                                         box-shadow: inset 2px 2px 4px var(--nd-shadow-dark), inset -2px -2px 4px var(--nd-shadow-light);"
                                    );
                                    format!(
                                        "width: 100%; padding: 12px 16px; text-align: left; \
                                         background: none; border: none; cursor: pointer; \
                                         font-size: 14px; color: inherit; border-radius: 8px; \
                                         display: flex; align-items: center; gap: 8px; \
                                         transition: background 0.15s ease; \
                                         {}",
                                        if values.contains(&option.value) {
                                            "background: linear-gradient(145deg, #7c3aed, #6d28d9);"
                                        } else {
                                            &inset_style
                                        }
                                    )
                                },
                                disabled: if option.disabled { "true" } else { "false" },
                                onclick: {
                                    let option_value = option.value.clone();
                                    let values = values.clone();
                                    let toggle_option = toggle_option.clone();
                                    move |_| {
                                        toggle_option(option_value.clone(), values.clone());
                                    }
                                },

                                // 复选框
                                div {
                                    style: {
                                        let inset = format!(
                                            "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                                             box-shadow: inset 2px 2px 4px var(--nd-shadow-dark), inset -2px -2px 4px var(--nd-shadow-light);"
                                        );
                                        format!(
                                            "width: 16px; height: 16px; border-radius: 4px; \
                                             display: flex; align-items: center; justify-content: center; \
                                             {}",
                                            if values.contains(&option.value) {
                                                "background: linear-gradient(145deg, #7c3aed, #6d28d9);"
                                            } else {
                                                &inset
                                            }
                                        )
                                    },
                                    if values.contains(&option.value) {
                                        span {
                                            style: "color: white; font-size: 10px;",
                                            "✓"
                                        }
                                    }
                                }

                                "{option.label}"
                            }
                        }
                    }
                }

                // 点击外部关闭
                div {
                    style: "position: fixed; inset: 0; z-index: 40;",
                    onclick: move |_| {
                        *is_open.write() = false;
                    },
                }
            }
        }
    }
}

// ==================== RangeSlider 范围滑块 ====================

/// RangeSlider
#[derive(Props, PartialEq, Clone)]
pub struct RangeSliderProps {
    /// Value
    pub value: i32,
    /// Change event
    pub on_change: EventHandler<i32>,
    /// Minimum value
    #[props(default = 0)]
    pub min: i32,
    /// Maximum value
    #[props(default = 100)]
    pub max: i32,
    /// Step size
    #[props(default = 1)]
    pub step: i32,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Whether to show the value
    #[props(default = true)]
    pub show_value: bool,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// RangeSlider Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     RangeSlider {
///         value: volume,
///         on_change: move |val| set_volume(val),
///         label: Some("Volume".to_string()),
///         min: 0,
///         max: 100,
///     }
/// }
/// ```
#[component]
pub fn RangeSlider(props: RangeSliderProps) -> Element {
    let class = props.class.unwrap_or_default();

    // 计算百分比
    let percentage = if props.max != props.min {
        ((props.value - props.min) as f64 / (props.max - props.min) as f64) * 100.0
    } else {
        0.0
    };

    rsx! {
        div {
            class: "nd-range-slider {class}",
            style: "display: flex; flex-direction: column; gap: 12px;",

            // 标签和值
            if props.label.is_some() || props.show_value {
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    if let Some(label_text) = props.label {
                        span {
                            style: "font-size: 14px; font-weight: 500; color: inherit;",
                            "{label_text}"
                        }
                    }
                    if props.show_value {
                        span {
                            style: "font-size: 14px; font-weight: 600; color: #7c3aed;",
                            "{props.value}"
                        }
                    }
                }
            }

            // 滑块容器
            div {
                style: "position: relative; height: 12px; border-radius: 6px;",

                // 轨道背景
                div {
                    class: "nd-range-slider-track",
                }

                // 进度条
                div {
                    class: "nd-range-slider-progress",
                    style: "width: {percentage}%;",
                }

                // 实际输入（隐藏但可交互）
                input {
                    r#type: "range",
                    value: props.value,
                    min: props.min,
                    max: props.max,
                    step: props.step,
                    disabled: if props.disabled { "true" } else { "false" },
                    "aria-valuemin": props.min,
                    "aria-valuemax": props.max,
                    "aria-valuenow": props.value,
                    style: "position: absolute; inset: 0; width: 100%; \
                            height: 100%; opacity: 0; cursor: pointer; z-index: 4;",
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<i32>() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 自定义滑块thumb - 添加 z-index: 3 确保在最上方
                div {
                    class: "nd-range-slider-thumb",
                    style: "left: {percentage}%;",
                }
            }
        }
    }
}

// ==================== StarRating 星级评分 ====================

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

// ==================== DatePicker 日期选择 ====================

/// DatePicker
#[derive(Props, PartialEq, Clone)]
pub struct DatePickerProps {
    /// Current selected date (ISO format)
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Placeholder text
    #[props(default = "Select a date".to_string())]
    pub placeholder: String,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Minimum date
    #[props(default)]
    pub min: Option<String>,
    /// Maximum date
    #[props(default)]
    pub max: Option<String>,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// DatePicker Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     DatePicker {
///         value: selected_date,
///         on_change: move |val| set_selected_date(Some(val)),
///         label: Some("Select Date".to_string()),
///     }
/// }
/// ```
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let value = props.value.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-date-picker {class}",
            style: "display: flex; flex-direction: column; gap: 8px;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit;",
                    "{label_text}"
                }
            }

            div {
                style: "position: relative;",

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 12px; z-index: -1; \
                         background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
                    ),
                }

                input {
                    r#type: "date",
                    value,
                    disabled: if props.disabled { "true" } else { "false" },
                    min: props.min.unwrap_or_default(),
                    max: props.max.unwrap_or_default(),
                    style: format!(
                        "width: 100%; padding: 12px 40px 12px 16px; border-radius: 12px; \
                         font-size: 14px; color: inherit; background: transparent; \
                         border: none; outline: none; cursor: pointer;"
                    ),
                    oninput: move |evt| {
                        let val = evt.value();
                        if !val.is_empty() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 日历图标
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%); pointer-events: none; \
                            font-size: 18px; opacity: 0.6;",
                    "📅"
                }
            }
        }
    }
}

// ==================== TimePicker 时间选择 ====================

/// TimePicker
#[derive(Props, PartialEq, Clone)]
pub struct TimePickerProps {
    /// Current selected time (HH:MM format)
    #[props(default)]
    pub value: Option<String>,
    /// Change event
    pub on_change: EventHandler<String>,
    /// Label text
    #[props(default)]
    pub label: Option<String>,
    /// Placeholder text
    #[props(default = "Select a time".to_string())]
    pub placeholder: String,
    /// Whether to disable
    #[props(default)]
    pub disabled: bool,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// TimePicker Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     TimePicker {
///         value: selected_time,
///         on_change: move |val| set_selected_time(Some(val)),
///         label: Some("Select Time".to_string()),
///     }
/// }
/// ```
#[component]
pub fn TimePicker(props: TimePickerProps) -> Element {
    let value = props.value.unwrap_or_default();
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-time-picker {class}",
            style: "display: flex; flex-direction: column; gap: 8px;",

            if let Some(label_text) = props.label {
                label {
                    style: "font-size: 14px; font-weight: 500; color: inherit;",
                    "{label_text}"
                }
            }

            div {
                style: "position: relative;",

                // 拟物化背景
                div {
                    style: format!(
                        "position: absolute; inset: 0; border-radius: 12px; z-index: -1; \
                         background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                         box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);",
                    ),
                }

                input {
                    r#type: "time",
                    value,
                    disabled: if props.disabled { "true" } else { "false" },
                    style: format!(
                        "width: 100%; padding: 12px 40px 12px 16px; border-radius: 12px; \
                         font-size: 14px; color: inherit; background: transparent; \
                         border: none; outline: none; cursor: pointer;"
                    ),
                    oninput: move |evt| {
                        let val = evt.value();
                        if !val.is_empty() {
                            props.on_change.call(val);
                        }
                    },
                }

                // 时钟图标
                span {
                    style: "position: absolute; right: 12px; top: 50%; \
                            transform: translateY(-50%); pointer-events: none; \
                            font-size: 18px; opacity: 0.6;",
                    "🕐"
                }
            }
        }
    }
}
