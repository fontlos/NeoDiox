//! Data Display Components
//!
//! Provides Accordion, Card, DataTable, BarChart, DonutChart

use crate::surfaces::{NeuFlat, NeuRaised};

use dioxus::prelude::*;

// ==================== Accordion 手风琴 ====================

/// Accordion - 单个可展开项
#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 是否展开
    #[props(default)]
    pub expanded: bool,
    /// 切换事件
    pub on_toggle: EventHandler<()>,
    /// 是否禁用
    #[props(default)]
    pub disabled: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
}

/// Accordion Component - 单个独立的可展开项
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Accordion {
///         title: "What is neuromorphic design?",
///         content: "Soft shadows and gradients...",
///         expanded: is_expanded,
///         on_toggle: move |_| toggle(),
///     }
/// }
/// ```
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-accordion {class}",

            NeuFlat {
                border_radius: 12,
                style: "overflow: hidden;",

                button {
                    r#type: "button",
                    class: "nd-accordion-trigger",
                    disabled: if props.disabled { "true" } else { "false" },
                    "aria-expanded": if props.expanded { "true" } else { "false" },
                    onclick: move |_| {
                        if !props.disabled {
                            props.on_toggle.call(());
                        }
                    },

                    span {
                        class: "nd-accordion-title",
                        "{props.title}"
                    }

                    span {
                        class: "nd-accordion-icon",
                        class: if props.expanded { "nd-accordion-icon-expanded" } else { "" },
                        "▼"
                    }
                }

                // 面板 - 始终渲染，用 max-height 控制动画
                div {
                    role: "region",
                    class: "nd-accordion-panel",
                    class: if props.expanded { "nd-accordion-panel-expanded" } else { "" },
                    style: if props.expanded {
                        "max-height: 500px;"
                    } else {
                        "max-height: 0;"
                    },

                    div {
                        class: "nd-accordion-content",
                        "{props.content}"
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
            class: "nd-card {class}",
            style: if props.hoverable {
                "transition: transform 0.3s ease, box-shadow 0.3s ease;"
            } else {
                ""
            },

            div {
                class: "nd-card-body",

                // 标题
                if let Some(title) = props.title {
                    h3 {
                        class: "nd-card-title",
                        "{title}"
                    }
                }

                // 描述
                if let Some(description) = props.description {
                    p {
                        class: "nd-card-description",
                        "{description}"
                    }
                }

                // 内容
                {props.children}
            }

            // 页脚
            if let Some(footer_content) = props.footer {
                div {
                    class: "nd-card-footer",
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
    let class = props.class.unwrap_or_default();
    let columns = props.columns.clone();
    let data = props.data.clone();
    let col_count = columns.len();

    rsx! {
        div {
            class: "nd-data-table {class}",

            // 表格容器
            div {
                style: format!(
                    "overflow-x: auto; border-radius: 12px; \
                     background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); \
                     box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
                ),

                table {
                    class: "nd-data-table-table",

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
                                                class: "nd-data-table-sort-icon",
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
                                class: "nd-data-table-row",

                                for column in &columns {
                                    td {
                                        class: "nd-data-table-cell",
                                        "{(column.render)(row)}"
                                    }
                                }
                            }
                        }

                        // 空状态
                        if data.is_empty() {
                            tr {
                                td {
                                    class: "nd-data-table-empty",
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
                class: "nd-bar-chart-empty",
                "No data available"
            }
        };
    }

    rsx! {
        div {
            class: "nd-bar-chart {class}",

            if let Some(title) = props.title {
                h3 {
                    class: "nd-bar-chart-title",
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
                        class: "nd-bar-item",

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
                            class: "nd-bar-label",
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
                class: "nd-donut-chart-empty",
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
            class: "nd-donut-chart nd-donut-chart-content {class}",

            if let Some(title) = props.title {
                h3 {
                    class: "nd-donut-chart-title",
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
                        class: "nd-donut-chart-svg",

                        // 背景圆环
                        circle {
                            cx: 50,
                            cy: 50,
                            r: 40,
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": 12,
                            class: "nd-donut-chart-bg",
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
                        class: "nd-donut-chart-center-text",
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
                class: "nd-donut-chart-legend",
                for (item, _, _, _, percentage) in &segments {
                    div {
                        class: "nd-donut-chart-legend-item",

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
                            class: "nd-donut-chart-legend-label",
                            "{item.label} {(percentage * 100.0).round()}%"
                        }
                    }
                }
            }
        }
    }
}
