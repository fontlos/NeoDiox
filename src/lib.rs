//! Neuromorphic UI component library for Dioxus.

pub mod button;
pub mod container;
pub mod data;
pub mod feedback;
pub mod form;
pub mod icon;
pub mod navigation;
pub mod style;
pub mod theme;

pub mod prelude {
    // 样式
    pub use crate::style::{RESET, RESET_STR, STYLE, STYLE_STR};
    // 主题
    pub use crate::theme::{ThemeVariant, ThemeVars, use_init_theme, use_theme, use_toggle_theme};
    // 容器组件
    pub use crate::container::{
        Accordion, Badge, Card, NeuFlat, NeuInset, NeuRaised, NeuRaisedSm, NeuSurface, SurfaceType,
    };
    // 按钮组件
    pub use crate::button::{Button, ButtonVariant};
    // 图标组件
    pub use crate::icon::Icon;
    // 表单组件
    pub use crate::form::{
        Checkbox, DatePicker, Dropdown, FileInfo, FileUpload, MultiSelect, Radio, Slider,
        StarRating, TextArea, TextInput, TimePicker, Toggle,
    };
    // 导航组件
    pub use crate::navigation::{
        BreadcrumbItem, Breadcrumbs, StepItem, StepStatus, Stepper, Tab, Tabs, TreeNode,
        TreeNodeKind, TreeView,
    };
    // 反馈组件
    pub use crate::feedback::{
        Alert, AlertType, Dots, Menu, MenuItem, Modal, ProgressBar, Skeleton, Spinner, Tip,
        TipPosition, ToastContainer, ToastManager, Toast, ToastPosition, ToastType,
    };
    // 数据组件
    pub use crate::data::{
        BarChart, BarData, Column, DonutChart, DonutData, Table, TableCell, TableRow,
    };
}
