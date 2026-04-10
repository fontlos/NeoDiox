//! 数据展示组件
//!
//! 提供 Badge、Tooltip、Accordion、Card、DataTable、BarChart、DonutChart

use crate::surfaces::{NeuFlat, NeuRaised};
use crate::theme::use_theme_config;
use dioxus::prelude::*;

// ==================== Badge 徽章 ====================

/// Badge Variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Danger,
    Warning,
    Info,
}

/// Badge
#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    /// Badge Variants
    #[props(default)]
    pub variant: BadgeVariant,
    /// Badge Text
    pub label: String,
    /// Icon (Optional)
    #[props(default)]
    pub icon: Option<String>,
    /// Whether to show the status dot
    #[props(default)]
    pub show_dot: bool,
    /// Status dot color
    #[props(default)]
    pub dot_color: Option<String>,
    /// Whether the badge is dismissible
    #[props(default)]
    pub dismissible: bool,
    /// Dismiss Event
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Badge Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Badge {
///         variant: BadgeVariant::Success,
///         label: "Active".to_string(),
///         show_dot: true,
///     }
/// }
/// ```
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    // 判断是否使用渐变背景
    let is_gradient = props.variant != BadgeVariant::Default;

    let (background, text_color) = if is_gradient {
        let (color1, color2) = match props.variant {
            BadgeVariant::Primary => ("#7c3aed", "#6d28d9"),
            BadgeVariant::Success => ("#22c55e", "#16a34a"),
            BadgeVariant::Danger => ("#ef4444", "#dc2626"),
            BadgeVariant::Warning => ("#eab308", "#ca8a04"),
            BadgeVariant::Info => ("#3b82f6", "#2563eb"),
            BadgeVariant::Default => ("#e5e7eb", "#d1d5db"),
        };
        (
            format!(
                "background: linear-gradient(145deg, {}, {});",
                color1, color2
            ),
            "white",
        )
    } else {
        // Default 变体使用主题色
        let tc = if theme.variant.is_dark() {
            "#d1d5db"
        } else {
            "#4b5563"
        };
        (
            format!(
                "background: linear-gradient(145deg, {}, {}); \
                 box-shadow: 4px 4px 8px {}, -4px -4px 8px {};",
                theme.bg_primary, theme.bg_secondary, theme.shadow_dark, theme.shadow_light
            ),
            tc,
        )
    };

    rsx! {
        span {
            class: "neu-badge {class}",
            style: format!(
                "display: inline-flex; align-items: center; gap: 6px; \
                 padding: 6px 12px; border-radius: 9999px; \
                 font-size: 12px; font-weight: 500; \
                 color: {text_color}; \
                 {background}"
            ),

            // 状态点
            if props.show_dot {
                span {
                    style: format!(
                        "width: 8px; height: 8px; border-radius: 50%; \
                         background: {};",
                        props.dot_color.unwrap_or_else(|| text_color.to_string())
                    ),
                }
            }

            // 图标
            if let Some(icon) = &props.icon {
                span {
                    style: "font-size: 12px;",
                    "{icon}"
                }
            }

            // 文本
            span {
                "{props.label}"
            }

            // 关闭按钮
            if props.dismissible {
                button {
                    r#type: "button",
                    style: "background: none; border: none; cursor: pointer; \
                            color: inherit; padding: 0; font-size: 12px; \
                            margin-left: 4px; opacity: 0.8;",
                    onclick: move |_| {
                        if let Some(handler) = props.on_dismiss {
                            handler.call(());
                        }
                    },
                    "✕"
                }
            }
        }
    }
}

// ==================== Tooltip 提示 ====================

/// Tooltip Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

/// Tooltip
#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    /// Tooltip Content
    pub content: String,
    /// Trigger Element
    pub children: Element,
    /// Tooltip Position
    #[props(default)]
    pub position: TooltipPosition,
    /// Whether to show the tooltip
    pub is_visible: bool,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Tooltip Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Tooltip {
///         content: "Home".to_string(),
///         is_visible: show_tooltip,
///         Button {
///             onclick: move |_| {},
///             "🏠"
///         }
///     }
/// }
/// ```
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let position_style = match props.position {
        TooltipPosition::Top => {
            "bottom: 100%; left: 50%; transform: translateX(-50%); margin-bottom: 8px;"
        }
        TooltipPosition::Bottom => {
            "top: 100%; left: 50%; transform: translateX(-50%); margin-top: 8px;"
        }
        TooltipPosition::Left => {
            "right: 100%; top: 50%; transform: translateY(-50%); margin-right: 8px;"
        }
        TooltipPosition::Right => {
            "left: 100%; top: 50%; transform: translateY(-50%); margin-left: 8px;"
        }
    };

    // 箭头样式
    let arrow_style = match props.position {
        TooltipPosition::Top => {
            "top: 100%; left: 50%; transform: translateX(-50%); \
                                 border: 6px solid transparent; border-top-color: #1f2937;"
        }
        TooltipPosition::Bottom => {
            "bottom: 100%; left: 50%; transform: translateX(-50%); \
                                    border: 6px solid transparent; border-bottom-color: #1f2937;"
        }
        TooltipPosition::Left => {
            "left: 100%; top: 50%; transform: translateY(-50%); \
                                  border: 6px solid transparent; border-left-color: #1f2937;"
        }
        TooltipPosition::Right => {
            "right: 100%; top: 50%; transform: translateY(-50%); \
                                   border: 6px solid transparent; border-right-color: #1f2937;"
        }
    };
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "neu-tooltip-container {class}",
            style: "position: relative; display: inline-block;",

            // 触发元素
            {props.children}

            // 提示内容
            if props.is_visible {
                div {
                    role: "tooltip",
                    style: format!(
                        "position: absolute; {position_style} \
                         padding: 8px 12px; border-radius: 8px; \
                         font-size: 12px; font-weight: 500; \
                         color: white; background: #1f2937; \
                         white-space: nowrap; z-index: 100; \
                         pointer-events: none;",
                    ),
                    "{props.content}"

                    // 箭头
                    div {
                        style: format!(
                            "position: absolute; {arrow_style}"
                        ),
                    }
                }
            }
        }
    }
}

// ==================== Accordion 手风琴 ====================

/// Accordion Item
#[derive(Clone, Debug, PartialEq)]
pub struct AccordionItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub disabled: bool,
}

/// Accordion
#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    /// Accordion Items
    pub items: Vec<AccordionItem>,
    /// Expanded Item IDs
    pub expanded_items: Vec<String>,
    /// Toggle Event
    pub on_toggle: EventHandler<String>,
    /// Whether to allow multiple items to be expanded
    #[props(default)]
    pub multiple: bool,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Accordion Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Accordion {
///         items: vec![
///             AccordionItem { id: "item1".to_string(), title: "Item 1".to_string(), content: "Content 1".to_string(), disabled: false },
///             AccordionItem { id: "item2".to_string(), title: "Item 2".to_string(), content: "Content 2".to_string(), disabled: false },
///         ],
///         expanded_items: expanded_items,
///         on_toggle: move |id| toggle_expanded(id),
///     }
/// }
/// ```
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let class = props.class.unwrap_or_default();
    let items = props.items.clone();
    let expanded_items = props.expanded_items.clone();
    let on_toggle = props.on_toggle.clone();

    rsx! {
        div {
            class: "neu-accordion {class}",
            style: "display: flex; flex-direction: column; gap: 12px;",

            for item in items {
                {
                    let item_id = item.id.clone();
                    let is_expanded = expanded_items.contains(&item_id);
                    let on_toggle = on_toggle.clone();
                    let disabled = item.disabled;
                    let item_id_clone = item_id.clone();
                    let item_id_clone2 = item_id.clone();
                    rsx! {
                        NeuFlat {
                            border_radius: 12,
                            class: "neu-accordion-item",
                            style: "overflow: hidden;",

                            div {
                                button {
                                    r#type: "button",
                                    class: "neu-accordion-trigger",
                                    disabled: if disabled { "true" } else { "false" },
                                    "aria-expanded": if is_expanded { "true" } else { "false" },
                                    "aria-controls": "accordion-panel-{item_id_clone}",
                                    style: format!(
                                        "width: 100%; padding: 16px 20px; display: flex; \
                                         align-items: center; justify-content: space-between; \
                                         background: none; border: none; cursor: {}; \
                                         font-size: 14px; font-weight: 500; color: inherit; \
                                         text-align: left;",
                                        if disabled { "default" } else { "pointer" }
                                    ),
                                    onclick: {
                                        let item_id = item_id_clone2.clone();
                                        move |_| {
                                            if !disabled {
                                                on_toggle.call(item_id.clone());
                                            }
                                        }
                                    },

                                    span {
                                        "{item.title}"
                                    }

                                    span {
                                        style: format!(
                                            "transition: transform 0.3s ease; font-size: 18px; \
                                             transform: {};",
                                            if is_expanded { "rotate(180deg)" } else { "rotate(0deg)" }
                                        ),
                                        "▼"
                                    }
                                }

                                if is_expanded {
                                    div {
                                        id: "accordion-panel-{item_id}",
                                        role: "region",
                                        style: "padding: 0 20px 16px 20px;",
                                        p {
                                            style: "font-size: 14px; color: inherit; opacity: 0.7; margin: 0; line-height: 1.6;",
                                            "{item.content}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==================== Card 卡片 ====================

/// Card
#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    /// Card Title
    #[props(default)]
    pub title: Option<String>,
    /// Card Subtitle/Description
    #[props(default)]
    pub description: Option<String>,
    /// Card Content
    pub children: Element,
    /// Footer Content
    #[props(default)]
    pub footer: Option<Element>,
    /// Whether to have a hover effect
    #[props(default)]
    pub hoverable: bool,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Card Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Card {
///         title: Some("Card Title".to_string()),
///         description: Some("Card description".to_string()),
///         NeuFlat {
///             border_radius: 8,
///             "Card content"
///         }
///     }
/// }
/// ```
#[component]
pub fn Card(props: CardProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        NeuRaised {
            border_radius: 24,
            class: "neu-card {class}",
            style: if props.hoverable {
                "transition: transform 0.3s ease, box-shadow 0.3s ease;"
            } else {
                ""
            },

            div {
                style: "padding: 24px;",

                // 标题
                if let Some(title) = props.title {
                    h3 {
                        style: "font-size: 18px; font-weight: 600; color: inherit; margin: 0 0 8px 0;",
                        "{title}"
                    }
                }

                // 描述
                if let Some(description) = props.description {
                    p {
                        style: "font-size: 14px; color: inherit; opacity: 0.7; margin: 0 0 16px 0; line-height: 1.5;",
                        "{description}"
                    }
                }

                // 内容
                {props.children}
            }

            // 页脚
            if let Some(footer_content) = props.footer {
                div {
                    style: "padding: 16px 24px; border-top: 1px solid rgba(128, 128, 128, 0.2);",
                    {footer_content}
                }
            }
        }
    }
}

// ==================== DataTable 数据表格 ====================

/// DataTable Column
#[derive(Clone, Debug)]
pub struct Column<T: Clone + PartialEq> {
    pub header: String,
    pub render: fn(&T) -> String,
    pub sortable: bool,
    pub width: Option<String>,
}

impl<T: Clone + PartialEq> PartialEq for Column<T> {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.sortable == other.sortable && self.width == other.width
    }
}

/// DataTable
#[derive(Props, PartialEq, Clone)]
pub struct DataTableProps<T: Clone + PartialEq + 'static> {
    /// Data to display
    pub data: Vec<T>,
    /// DataTable Column
    pub columns: Vec<Column<T>>,
    /// Current sort column
    #[props(default)]
    pub sort_column: Option<usize>,
    /// Sort Direction
    #[props(default)]
    pub sort_direction: SortDirection,
    /// Sort Event
    #[props(default)]
    pub on_sort: Option<EventHandler<(usize, SortDirection)>>,
    /// Search Text
    #[props(default)]
    pub search_query: Option<String>,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Sort Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

/// DataTable Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     DataTable {
///         data: users,
///         columns: vec![
///             Column { header: "Name".to_string(), render: |u| u.name.clone(), sortable: true, width: None },
///             Column { header: "Email".to_string(), render: |u| u.email.clone(), sortable: true, width: None },
///         ],
///         sort_column: None,
///         sort_direction: SortDirection::Asc,
///         on_sort: Some(move |(idx, dir)| handle_sort(idx, dir)),
///     }
/// }
/// ```
#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(props: DataTableProps<T>) -> Element {
    let theme = use_theme_config();
    let class = props.class.unwrap_or_default();
    let columns = props.columns.clone();
    let data = props.data.clone();
    let col_count = columns.len();

    rsx! {
        div {
            class: "neu-data-table {class}",

            // 表格容器
            div {
                style: format!(
                    "overflow-x: auto; border-radius: 12px; \
                     background: linear-gradient(145deg, {}, {}); \
                     box-shadow: inset 4px 4px 8px {}, inset -4px -4px 8px {};",
                    theme.bg_secondary, theme.bg_primary,
                    theme.shadow_dark, theme.shadow_light
                ),

                table {
                    style: "width: 100%; border-collapse: collapse; font-size: 14px;",

                    // 表头
                    thead {
                        tr {
                            for (col_idx, column) in columns.iter().enumerate() {
                                th {
                                    scope: "col",
                                    style: format!(
                                        "padding: 16px; text-align: left; font-weight: 500; \
                                         color: inherit; border-bottom: 1px solid rgba(128, 128, 128, 0.2); \
                                         {}",
                                        if column.sortable { "cursor: pointer; user-select: none;" } else { "" }
                                    ),
                                    onclick: {
                                        let column = column.clone();
                                        let on_sort = props.on_sort.clone();
                                        let sort_column = props.sort_column;
                                        let sort_direction = props.sort_direction;
                                        move |_| {
                                            if column.sortable {
                                                if let Some(handler) = on_sort {
                                                    let new_direction = if sort_column == Some(col_idx) {
                                                        match sort_direction {
                                                            SortDirection::Asc => SortDirection::Desc,
                                                            SortDirection::Desc => SortDirection::Asc,
                                                        }
                                                    } else {
                                                        SortDirection::Asc
                                                    };
                                                    handler.call((col_idx, new_direction));
                                                }
                                            }
                                        }
                                    },

                                    div {
                                        style: "display: flex; align-items: center; gap: 8px;",
                                        "{column.header}"

                                        // 排序图标
                                        if column.sortable {
                                            span {
                                                style: "font-size: 12px; opacity: 0.6;",
                                                if props.sort_column == Some(col_idx) {
                                                    match props.sort_direction {
                                                        SortDirection::Asc => "↑",
                                                        SortDirection::Desc => "↓",
                                                    }
                                                } else {
                                                    "↕"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 表体
                    tbody {
                        for row in &data {
                            tr {
                                style: "border-bottom: 1px solid rgba(128, 128, 128, 0.1); \
                                        transition: background 0.15s ease;",

                                for column in &columns {
                                    td {
                                        style: "padding: 16px; color: inherit;",
                                        "{(column.render)(row)}"
                                    }
                                }
                            }
                        }

                        // 空状态
                        if data.is_empty() {
                            tr {
                                td {
                                    style: "padding: 32px; text-align: center; color: inherit; opacity: 0.6;",
                                    colspan: col_count,
                                    "No data available"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==================== BarChart 柱状图 ====================

/// Bar Chart Data Item
#[derive(Clone, Debug, PartialEq)]
pub struct BarData {
    pub label: String,
    pub value: f64,
    pub color: Option<(String, String)>,
}

/// Bar Chart
#[derive(Props, PartialEq, Clone)]
pub struct BarChartProps {
    /// Chart Data
    pub data: Vec<BarData>,
    /// Chart Title
    #[props(default)]
    pub title: Option<String>,
    /// Chart Height
    #[props(default = 200)]
    pub height: u32,
    /// Whether to display animation
    #[props(default = true)]
    pub animated: bool,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Bar Chart Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     BarChart {
///         data: vec![
///             BarData { label: "Mon".to_string(), value: 60.0, color: None },
///             BarData { label: "Tue".to_string(), value: 80.0, color: None },
///             BarData { label: "Wed".to_string(), value: 45.0, color: None },
///         ],
///         title: Some("Weekly Activity".to_string()),
///     }
/// }
/// ```
#[component]
pub fn BarChart(props: BarChartProps) -> Element {
    // let theme = use_theme_config();
    let class = props.class.unwrap_or_default();

    // 计算最大值
    let max_value = props.data.iter().map(|d| d.value).fold(0.0f64, f64::max);

    if max_value == 0.0 {
        return rsx! {
            div {
                style: "text-align: center; padding: 32px; opacity: 0.6;",
                "No data available"
            }
        };
    }

    rsx! {
        div {
            class: "neu-bar-chart {class}",

            if let Some(title) = props.title {
                h3 {
                    style: "font-size: 16px; font-weight: 600; color: inherit; margin: 0 0 16px 0;",
                    "{title}"
                }
            }

            // 图表容器
            div {
                role: "img",
                style: format!(
                    "display: flex; align-items: flex-end; justify-content: space-between; \
                     height: {}px; width: 100%; gap: 16px; padding: 0 16px;",
                    props.height
                ),

                for (index, item) in props.data.iter().enumerate() {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; flex: 1;",

                        // 柱体
                        div {
                            style: {
                                let base = format!(
                                    "width: 100%; border-radius: 8px 8px 0 0; \
                                     background: linear-gradient(180deg, {}, {}); \
                                     height: {}%; \
                                     transition: height 0.8s ease;",
                                    item.color.as_ref().map(|c| c.0.clone()).unwrap_or_else(|| "#a855f7".to_string()),
                                    item.color.as_ref().map(|c| c.1.clone()).unwrap_or_else(|| "#7c3aed".to_string()),
                                    (item.value / max_value * 100.0).round()
                                );
                                if props.animated {
                                    format!(
                                        "{} animation: grow-up 0.8s ease-out {}ms both;",
                                        base,
                                        index * 100
                                    )
                                } else {
                                    base
                                }
                            },
                        }

                        // 标签
                        span {
                            style: "font-size: 12px; color: inherit; opacity: 0.7; margin-top: 8px;",
                            "{item.label}"
                        }
                    }
                }
            }
        }
    }
}

// ==================== DonutChart 环形图 ====================

/// Donut Chart Data Item
#[derive(Clone, Debug, PartialEq)]
pub struct DonutData {
    pub label: String,
    pub value: f64,
    pub color: (String, String),
}

/// Donut Chart
#[derive(Props, PartialEq, Clone)]
pub struct DonutChartProps {
    /// Chart Data
    pub data: Vec<DonutData>,
    /// Chart Title
    #[props(default)]
    pub title: Option<String>,
    /// Chart Size
    #[props(default = 160)]
    pub size: u32,
    /// Center Text
    #[props(default)]
    pub center_text: Option<String>,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Donut Chart Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     DonutChart {
///         data: vec![
///             DonutData { label: "Desktop".to_string(), value: 50.0, color: ("#7c3aed".to_string(), "#a855f7".to_string()) },
///             DonutData { label: "Mobile".to_string(), value: 30.0, color: ("#10b981".to_string(), "#34d399".to_string()) },
///             DonutData { label: "Tablet".to_string(), value: 20.0, color: ("#f59e0b".to_string(), "#fbbf24".to_string()) },
///         ],
///         center_text: Some("100%".to_string()),
///     }
/// }
/// ```
#[component]
pub fn DonutChart(props: DonutChartProps) -> Element {
    let total: f64 = props.data.iter().map(|d| d.value).sum();
    let class = props.class.unwrap_or_default();

    if total == 0.0 {
        return rsx! {
            div {
                style: "text-align: center; padding: 32px; opacity: 0.6;",
                "No data available"
            }
        };
    }

    // 计算每个扇形的 dasharray 和 dashoffset
    let circumference = 2.0 * std::f64::consts::PI * 40.0;
    let mut cumulative_offset = 0.0;

    let segments: Vec<_> = props
        .data
        .iter()
        .map(|item| {
            let percentage = item.value / total;
            let dash_length = percentage * circumference;
            let gap_length = circumference - dash_length;
            let dashoffset = -cumulative_offset;
            cumulative_offset += dash_length;

            (item, dash_length, gap_length, dashoffset, percentage)
        })
        .collect();

    rsx! {
        div {
            class: "neu-donut-chart {class}",
            style: "display: flex; flex-direction: column; align-items: center;",

            if let Some(title) = props.title {
                h3 {
                    style: "font-size: 16px; font-weight: 600; color: inherit; margin: 0 0 16px 0;",
                    "{title}"
                }
            }

            // 环形图
            div {
                role: "img",
                style: format!("width: {}px; height: {}px; position: relative;", props.size, props.size),

                div {
                    style: "width: 100%; height: 100%; transform: rotate(-90deg);",
                    svg {
                        view_box: "0 0 100 100",
                        style: "width: 100%; height: 100%;",

                        // 背景圆环
                        circle {
                            cx: 50,
                            cy: 50,
                            r: 40,
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": 12,
                            style: "opacity: 0.1;",
                        }

                        // 数据扇形
                        for (item, dash_length, gap_length, dashoffset, _) in &segments {
                            circle {
                                cx: 50,
                                cy: 50,
                                r: 40,
                                fill: "none",
                                stroke: {
                                    let gid = format!("gradient-{}-{}", item.color.0.replace('#', ""), item.color.1.replace('#', ""));
                                    format!("url(#{gid})")
                                },
                                "stroke-width": 12,
                                "stroke-dasharray": format!("{dash_length} {gap_length}"),
                                "stroke-dashoffset": format!("{dashoffset}"),
                                "stroke-linecap": "round",
                            }
                        }

                        // 渐变定义
                        defs {
                            for (item, _, _, _, _) in &segments {
                                linearGradient {
                                    id: {
                                        format!("gradient-{}-{}", item.color.0.replace('#', ""), item.color.1.replace('#', ""))
                                    },
                                    x1: "0%",
                                    y1: "0%",
                                    x2: "100%",
                                    y2: "0%",
                                    stop {
                                        offset: "0%",
                                        "stop-color": "{item.color.0}",
                                    }
                                    stop {
                                        offset: "100%",
                                        "stop-color": "{item.color.1}",
                                    }
                                }
                            }
                        }
                    }
                }

                // 中心文本
                div {
                    style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center;",
                    span {
                        style: "font-size: 24px; font-weight: 600; color: inherit;",
                        if let Some(center_text) = props.center_text {
                            "{center_text}"
                        } else {
                            "{total.round()}"
                        }
                    }
                }
            }

            // 图例
            div {
                style: "display: flex; gap: 16px; margin-top: 16px; flex-wrap: wrap; justify-content: center;",
                for (item, _, _, _, percentage) in &segments {
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",

                        // 颜色指示
                        div {
                            style: format!(
                                "width: 12px; height: 12px; border-radius: 6px; \
                                 background: linear-gradient(145deg, {}, {});",
                                item.color.0, item.color.1
                            ),
                        }

                        // 标签
                        span {
                            style: "font-size: 12px; color: inherit; opacity: 0.7;",
                            "{item.label} {(percentage * 100.0).round()}%"
                        }
                    }
                }
            }
        }
    }
}
