use dioxus::prelude::*;
use neo_diox::prelude::*;

#[component]
pub fn Data() -> Element {
    rsx! {
        div {
            class: "page",
            style: "grid-template-columns: repeat(2, minmax(0, 1fr));",

            TablePanel {}
            BarPanel {}
            DonutPanel {}
        }
    }
}

#[component]
fn TablePanel() -> Element {
    rsx! {
        NeuRaised { class: "panel big-panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:table",
                    "data-width": 20,
                }
                "Data Table"
            }
            Table {
                columns: vec![
                    Column {
                        header: "Name".to_string(),
                        width: None,
                    },
                    Column {
                        header: "Email".to_string(),
                        width: None,
                    },
                    Column {
                        header: "Status".to_string(),
                        width: None,
                    },
                    Column {
                        header: "Role".to_string(),
                        width: None,
                    },
                ],
                rows: vec![
                    TableRow {
                        cells: vec![
                            TableCell {
                                content: "John Doe".to_string(),
                                class: None,
                            },
                            TableCell {
                                content: "john@example.com".to_string(),
                                class: None,
                            },
                            TableCell {
                                content: r#"<span class="nd-badge nd-badge-green">Active</span>"#
                                    .to_string(),
                                class: None,
                            },
                            TableCell {
                                content: "Admin".to_string(),
                                class: None,
                            },
                        ],
                    },
                    TableRow {
                        cells: vec![
                            TableCell {
                                content: "Jane Smith".to_string(),
                                class: None,
                            },
                            TableCell {
                                content: "jane@example.com".to_string(),
                                class: None,
                            },
                            TableCell {
                                content: r#"<span class="nd-badge nd-badge-yellow">Pending</span>"#
                                    .to_string(),
                                class: None,
                            },
                            TableCell {
                                content: "Editor".to_string(),
                                class: None,
                            },
                        ],
                    },
                    TableRow {
                        cells: vec![
                            TableCell {
                                content: "Bob Johnson".to_string(),
                                class: None,
                            },
                            TableCell {
                                content: "bob@example.com".to_string(),
                                class: None,
                            },
                            TableCell {
                                content: r#"<span class="nd-badge nd-badge-red">Inactive</span>"#
                                    .to_string(),
                                class: None,
                            },
                            TableCell {
                                content: "Viewer".to_string(),
                                class: None,
                            },
                        ],
                    },
                ],
            }
        }
    }
}

#[component]
fn BarPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:bar-chart-3",
                    "data-width": 20,
                }
                "Bar Chart"
            }
            BarChart {
                data: vec![
                    BarData {
                        label: "Mon".to_string(),
                        value: 60.0,
                        color: None,
                    },
                    BarData {
                        label: "Tue".to_string(),
                        value: 80.0,
                        color: None,
                    },
                    BarData {
                        label: "Wed".to_string(),
                        value: 45.0,
                        color: None,
                    },
                    BarData {
                        label: "Thu".to_string(),
                        value: 90.0,
                        color: None,
                    },
                    BarData {
                        label: "Fri".to_string(),
                        value: 70.0,
                        color: None,
                    },
                ],
                height: 200,
            }
        }
    }
}

#[component]
fn DonutPanel() -> Element {
    rsx! {
        NeuRaised { class: "panel",
            h2 {
                Icon {
                    class: "iconify",
                    size: 20,
                    "data-icon": "lucide:pie-chart",
                    "data-width": 20,
                }
                "Donut Chart"
            }
            DonutChart {
                data: vec![
                    DonutData {
                        label: "Desktop".to_string(),
                        value: 50.0,
                        color: ("#7c3aed".to_string(), "#a855f7".to_string()),
                    },
                    DonutData {
                        label: "Mobile".to_string(),
                        value: 30.0,
                        color: ("#10b981".to_string(), "#34d399".to_string()),
                    },
                    DonutData {
                        label: "Tablet".to_string(),
                        value: 20.0,
                        color: ("#f59e0b".to_string(), "#fbbf24".to_string()),
                    },
                ],
                center_text: Some("100%".to_string()),
            }
        }
    }
}
