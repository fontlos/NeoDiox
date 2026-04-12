//! Neuromorphic UI component library for Dioxus.

pub mod advanced;
pub mod button;
pub mod container;
pub mod data;
pub mod feedback;
pub mod icon;
pub mod inputs;
pub mod label;
pub mod navigation;
pub mod selector;
pub mod styles;
pub mod surfaces;
pub mod theme;

pub mod prelude {
    // 样式
    pub use crate::styles::{NORMALIZE, NORMALIZE_S, STYLE, STYLE_S};
    // 主题
    pub use crate::theme::{ThemeVariant, ThemeVars, use_theme, use_toggle_theme};
    // 容器组件
    pub use crate::surfaces::{NeuFlat, NeuInset, NeuRaised, NeuRaisedSm, NeuSurface, SurfaceType};
    // 按钮组件
    pub use crate::button::{Button, ButtonVariant};
    // 输入组件
    pub use crate::inputs::{InputSize, ResizeMode, SearchInput, TextArea, TextInput};
    pub use crate::label::{Badge, Tooltip, TooltipPosition};
    // 选择控件
    pub use crate::selector::{
        Checkbox, DatePicker, Dropdown, DropdownOption, MultiSelect, MultiSelectOption, Radio,
        RadioGroup, RadioOption, Slider, StarRating, TimePicker, Toggle,
    };
    // 反馈组件
    pub use crate::feedback::{
        Alert, AlertType, Modal, ModalSize, ProgressBar, Skeleton, Spinner, ToastContainer,
        ToastMessage, ToastPosition, ToastType,
    };
    // 导航组件
    pub use crate::container::Accordion;
    pub use crate::navigation::{
        BreadcrumbItem, Breadcrumbs, StepItem, StepStatus, Stepper, StepperDirection, TabOption,
        Tabs,
    };
    // 数据组件
    pub use crate::data::{BarChart, BarData, Column, DonutChart, DonutData, SortDirection, Table};
    // 高级组件
    pub use crate::advanced::{
        ContextMenu, FeatureCard, FileInfo, FileUpload, MenuItem, TreeNode, TreeView,
    };
    // 图标组件
    pub use crate::icon::Icon;
}
