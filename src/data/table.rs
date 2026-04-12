use dioxus::prelude::*;

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

/// Table
#[derive(Props, PartialEq, Clone)]
pub struct TableProps<T: Clone + PartialEq + 'static> {
    /// Data to display
    pub data: Vec<T>,
    /// Table Column
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

/// Table Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Table {
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
pub fn Table<T: Clone + PartialEq + 'static>(props: TableProps<T>) -> Element {
    let class = props.class.unwrap_or_default();
    let columns = props.columns.clone();
    let data = props.data.clone();
    let col_count = columns.len();

    rsx! {
        div {
            class: "nd-table {class}",

            // 表格容器
            div {
                style: format!(
                    "overflow-x: auto; border-radius: 12px; \
                     background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); \
                     box-shadow: inset 4px 4px 8px var(--nd-shadow-dark), inset -4px -4px 8px var(--nd-shadow-light);"
                ),

                table {
                    class: "nd-table-table",

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
                                                class: "nd-table-sort-icon",
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
                                class: "nd-table-row",

                                for column in &columns {
                                    td {
                                        class: "nd-table-cell",
                                        "{(column.render)(row)}"
                                    }
                                }
                            }
                        }

                        // 空状态
                        if data.is_empty() {
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
