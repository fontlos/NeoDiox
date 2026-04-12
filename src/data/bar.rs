use dioxus::prelude::*;

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
