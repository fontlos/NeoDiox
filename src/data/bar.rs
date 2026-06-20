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
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,

    /// Chart Data
    pub data: Vec<BarData>,
    /// Chart Height
    #[props(default = 200)]
    pub height: u32,
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
///     }
/// }
/// ```
#[component]
pub fn BarChart(props: BarChartProps) -> Element {
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
            height: "{props.height}px",

            for (index, item) in props.data.iter().enumerate() {
                div {
                    class: "nd-bar-item",
                    height: format!("{}%", (item.value / max_value * 100.0).round()),

                    // 柱体 - 使用 CSS 类
                    div {
                        class: "nd-bar-rectangle",
                        style: {
                            let mut styles = vec![];

                            if let Some((start, end)) = &item.color {
                                styles.push(format!(
                                    "background: linear-gradient(180deg, {}, {})",
                                    start, end
                                ));
                            }

                            styles.push(format!(
                                "animation-delay: {}ms",
                                index * 100
                            ));

                            styles.join("; ")
                        }
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
