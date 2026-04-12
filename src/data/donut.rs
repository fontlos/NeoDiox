use dioxus::prelude::*;

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
