//! Neuromorphic UI component library for Dioxus.

pub mod advanced;
pub mod button;
pub mod container;
pub mod data;
pub mod feedback;
pub mod icon;
pub mod input;
pub mod navigation;
pub mod selector;
pub mod style;
pub mod theme;

pub mod prelude {
    // 样式
    pub use crate::style::{STYLE, STYLE_STR, RESET, RESET_STR};
    // 主题
    pub use crate::theme::{ThemeVariant, ThemeVars, use_theme, use_toggle_theme};
    // 容器组件
    pub use crate::container::{
        Accordion, Badge, Card, NeuFlat, NeuInset, NeuRaised, NeuRaisedSm, NeuSurface, SurfaceType,
    };
    // 按钮组件
    pub use crate::button::{Button, ButtonVariant};
    // 图标组件
    pub use crate::icon::Icon;
    // 输入组件
    pub use crate::input::{TextArea, TextInput};
    // 选择组件
    pub use crate::selector::{
        Checkbox, DatePicker, Dropdown, DropdownOption, MultiSelect, MultiSelectOption, Radio,
        RadioGroup, RadioOption, Slider, StarRating, TimePicker, Toggle,
    };
    // 导航组件
    pub use crate::navigation::{
        BreadcrumbItem, Breadcrumbs, StepItem, StepStatus, Stepper, StepperDirection, TabOption,
        Tabs,
    };
    // 反馈组件
    pub use crate::feedback::{
        Alert, AlertType, DotsLoading, Modal, ModalSize, ProgressBar, Skeleton, Spinner, Tip,
        TipPosition, ToastContainer, ToastMessage, ToastPosition, ToastType,
    };
    // 数据组件
    pub use crate::data::{BarChart, BarData, Column, DonutChart, DonutData, SortDirection, Table};
    // 高级组件
    pub use crate::advanced::{FileInfo, FileUpload, Menu, MenuItem, TreeNode, TreeView};
}
