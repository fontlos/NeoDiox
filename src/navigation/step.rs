use dioxus::prelude::*;

/// Step Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    #[default]
    Pending,
    Active,
    Completed,
    Current,
}

impl StepStatus {
    fn style(&self) -> String {
        match self {
            Self::Completed | Self::Current => format!(
                "background: linear-gradient(145deg, #7c3aed, #6d28d9); \
                 color: white; box-shadow: 4px 4px 8px var(--nd-shadow-dark), -4px -4px 8px var(--nd-shadow-light);",
            ),
            Self::Active => format!(
                "background: linear-gradient(145deg, var(--nd-bg-primary), var(--nd-bg-secondary)); \
                 box-shadow: inset 3px 3px 6px var(--nd-shadow-dark), inset -3px -3px 6px var(--nd-shadow-light);",
            ),
            Self::Pending => format!(
                "background: linear-gradient(145deg, var(--nd-bg-secondary), var(--nd-bg-primary)); \
                 box-shadow: inset 3px 3px 6px var(--nd-shadow-dark), inset -3px -3px 6px var(--nd-shadow-light);",
            ),
        }
    }

    fn icon(&self) -> Option<&'static str> {
        match self {
            Self::Completed | Self::Current => Some("✓"),
            _ => None,
        }
    }
}

/// Step Item
#[derive(Clone, Debug, PartialEq)]
pub struct StepItem {
    pub label: String,
    pub status: StepStatus,
    pub icon: Option<String>,
}

/// Stepper Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl StepperDirection {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Horizontal => "nd-stepper",
            Self::Vertical => "nd-stepper nd-stepper-vertical",
        }
    }
}

/// Stepper
#[derive(Props, PartialEq, Clone)]
pub struct StepperProps {
    /// Steps
    pub steps: Vec<StepItem>,
    /// Currently active step index
    #[props(default)]
    pub current_step: Option<usize>,
    /// Direction
    #[props(default)]
    pub direction: StepperDirection,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Stepper Component
#[component]
pub fn Stepper(props: StepperProps) -> Element {
    let class = props.class.unwrap_or_default();
    let stepper_class = props.direction.css_class();

    rsx! {
        div {
            class: "{stepper_class} {class}",
            role: "navigation",
            for (index, step) in props.steps.iter().enumerate() {
                div {
                    class: if props.direction == StepperDirection::Vertical {
                        "nd-step nd-step-vertical"
                    } else {
                        "nd-step"
                    },

                    // 步骤指示器
                    div {
                        class: "nd-step-indicator",
                        style: step.status.style(),
                        if let Some(icon) = step.icon.as_ref() {
                            "{icon}"
                        } else if let Some(icon) = step.status.icon() {
                            "{icon}"
                        } else {
                            "{index + 1}"
                        }
                    }

                    // 步骤内容
                    div { class: "nd-step-content",
                        p { class: "nd-step-title", "{step.label}" }
                    }
                }
            }
        }
    }
}
