use dioxus::prelude::*;

/// Table Column
#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub header: String,
    pub width: Option<String>,
}

/// Table Row Cell
#[derive(Clone, Debug, PartialEq)]
pub struct TableCell {
    /// Cell content (HTML string or plain text)
    pub content: String,
    /// Optional custom class for this cell
    pub class: Option<String>,
}

/// Table Row
#[derive(Clone, Debug, PartialEq)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
}

/// Table Props
#[derive(Props, PartialEq, Clone)]
pub struct TableProps {
    /// Table Columns
    pub columns: Vec<Column>,
    /// Table Rows
    pub rows: Vec<TableRow>,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// Table Component - Simple data table with neu-inset container
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Table {
///         columns: vec![
///             Column { header: "Name".to_string(), width: None },
///             Column { header: "Email".to_string(), width: None },
///             Column { header: "Status".to_string(), width: None },
///             Column { header: "Role".to_string(), width: None },
///         ],
///         rows: vec![
///             TableRow { cells: vec![
///                 TableCell { content: "John Doe".to_string(), class: None },
///                 TableCell { content: "john@example.com".to_string(), class: None },
///                 TableCell { content: "<span class='badge-active'>Active</span>".to_string(), class: None },
///                 TableCell { content: "Admin".to_string(), class: None },
///             ]},
///         ],
///     }
/// }
/// ```
#[component]
pub fn Table(props: TableProps) -> Element {
    let class = props.class.unwrap_or_default();
    let col_count = props.columns.len();

    rsx! {
        div {
            class: "nd-table {class}",

            // neu-inset 容器
            div {
                class: "nd-table-container",

                table {
                    class: "nd-table-table",
                    role: "table",

                    // 表头
                    thead {
                        tr {
                            for column in &props.columns {
                                th {
                                    scope: "col",
                                    class: "nd-table-header",
                                    style: if let Some(width) = &column.width {
                                        format!("width: {width};")
                                    } else {
                                        "".to_string()
                                    },
                                    "{column.header}"
                                }
                            }
                        }
                    }

                    // 表体
                    tbody {
                        for row in &props.rows {
                            tr {
                                class: "nd-table-row",
                                for cell in &row.cells {
                                    td {
                                        class: "nd-table-cell",
                                        dangerous_inner_html: "{cell.content}",
                                    }
                                }
                            }
                        }

                        // 空状态
                        if props.rows.is_empty() {
                            tr {
                                td {
                                    class: "nd-table-empty",
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
