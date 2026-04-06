//! Neuromorphic UI component library for Dioxus.

pub mod advanced;
pub mod button;
pub mod data;
pub mod feedback;
pub mod form_controls;
pub mod inputs;
pub mod navigation;
pub mod selectors;
pub mod styles;
pub mod surfaces;
pub mod theme;

pub mod prelude {
    // 样式
    pub use crate::styles::STYLE;
    // 主题
    pub use crate::theme::{ThemeConfig, ThemeVariant, use_theme_config, use_toggle_theme};
    // 容器组件
    pub use crate::surfaces::{NeuFlat, NeuInset, NeuRaised, NeuRaisedSm, NeuSurface, SurfaceType};
    // 按钮组件
    pub use crate::button::{Button, ButtonVariant};
    // 表单控件
    pub use crate::form_controls::{Checkbox, Radio, RadioGroup, RadioOption, Toggle};
    // 输入组件
    pub use crate::inputs::{InputSize, ResizeMode, SearchInput, TextArea, TextInput};
    // 选择器
    pub use crate::selectors::{
        DatePicker, Dropdown, DropdownOption, MultiSelect, MultiSelectOption, RangeSlider,
        StarRating, TimePicker,
    };
    // 反馈组件
    pub use crate::feedback::{
        AlertType, Alert, Modal, ModalSize, ProgressBar, Skeleton, Spinner, ToastContainer, ToastMessage,
        ToastPosition, ToastType,
    };
    // 导航组件
    pub use crate::navigation::{
        BreadcrumbItem, Breadcrumbs, StepItem, StepStatus, Stepper, StepperDirection, TabOption,
        Tabs,
    };
    // 数据展示
    pub use crate::data::{
        Accordion, AccordionItem, Badge, BadgeVariant, BarChart, BarData, Card, Column, DataTable,
        DonutChart, DonutData, SortDirection, Tooltip, TooltipPosition,
    };
    // 高级组件
    pub use crate::advanced::{
        ContextMenu, FeatureCard, FileInfo, FileUpload, MenuItem, TreeNode, TreeView,
    };
}
