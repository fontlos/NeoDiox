use dioxus::prelude::*;

// ==================== FileUpload 文件上传 ====================

/// File Information
#[derive(Clone, Debug, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub file_type: String,
}

/// File Upload
#[derive(Props, PartialEq, Clone)]
pub struct FileUploadProps {
    /// List of Uploaded Files
    pub files: Vec<FileInfo>,
    /// Add File Event
    pub on_add: EventHandler<Vec<FileInfo>>,
    /// Remove File Event
    pub on_remove: EventHandler<usize>,
    /// Accepted File Types
    #[props(default)]
    pub accept: Option<String>,
    /// Whether to support multiple file selection
    #[props(default = true)]
    pub multiple: bool,
    /// Maximum File Size (bytes)
    #[props(default)]
    pub max_size: Option<u64>,
    /// Drag and Drop Area Text
    #[props(default = "Drop files here or click to upload".to_string())]
    pub drop_text: String,
    /// Custom Class Name
    #[props(default)]
    pub class: Option<String>,
}

/// File Upload Component
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     FileUpload {
///         files: uploaded_files,
///         on_add: move |new_files| add_files(new_files),
///         on_remove: move |idx| remove_file(idx),
///         accept: Some("image/*,.pdf".to_string()),
///         max_size: Some(10 * 1024 * 1024), // 10MB
///     }
/// }
/// ```
#[component]
pub fn FileUpload(props: FileUploadProps) -> Element {
    let class = props.class.unwrap_or_default();

    rsx! {
        div {
            class: "nd-file-upload {class}",

            // 拖拽区域
            div {
                class: "nd-file-drop-zone",
                onclick: move |_| {
                    // 在实际应用中，这里应该触发文件选择对话框
                },

                div {
                    class: "nd-file-drop-content",

                    // 上传图标
                    div {
                        class: "nd-file-upload-icon",
                        span {
                            "📤"
                        }
                    }

                    p {
                        class: "nd-file-upload-title",
                        "{props.drop_text}"
                    }

                    p {
                        class: "nd-file-upload-help",
                        "PNG, JPG, PDF up to 10MB"
                    }
                }
            }

            // 文件列表
            if !props.files.is_empty() {
                div {
                    class: "nd-file-list",
                    for (index, file) in props.files.iter().enumerate() {
                        div {
                            class: "nd-file-item",

                            // 文件图标
                            span {
                                class: "nd-file-item-icon",
                                "📄"
                            }

                            // 文件信息
                            div {
                                class: "nd-file-item-content",
                                p {
                                    class: "nd-file-item-name",
                                    "{file.name}"
                                }
                                p {
                                    class: "nd-file-item-size",
                                    "{format_file_size(file.size)}"
                                }
                            }

                            // 移除按钮
                            button {
                                r#type: "button",
                                class: "nd-file-item-remove",
                                onclick: move |_| {
                                    props.on_remove.call(index);
                                },
                                "✕"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_file_size(size: u64) -> String {
    if size < 1024 {
        format!("{size} B")
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    }
}
