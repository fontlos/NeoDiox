use dioxus::prelude::*;

/// Step Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepStatus {
    Completed,
    #[default]
    Current,
    Pending,
}

/// Step Item
#[derive(Clone, Debug, PartialEq)]
pub struct StepItem {
    pub label: String,
    pub status: StepStatus,
}

/// Stepper Props
#[derive(Props, PartialEq, Clone)]
pub struct StepperProps {
    /// Steps
    pub steps: Vec<StepItem>,
    /// Currently active step index
    #[props(default)]
    pub current_step: usize,
    /// Custom class name
    #[props(default)]
    pub class: Option<String>,
}

/// Stepper Component - horizontal progress stepper with connectors
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Stepper {
///         steps: vec![
///             StepItem { label: "Account".to_string(), status: StepStatus::Completed },
///             StepItem { label: "Details".to_string(), status: StepStatus::Current },
///             StepItem { label: "Confirm".to_string(), status: StepStatus::Pending },
///         ],
///         current_step: 1,
///     }
/// }
/// ```
#[component]
pub fn Stepper(props: StepperProps) -> Element {
    let class = props.class.unwrap_or_default();
    let total = props.steps.len();

    rsx! {
        div {
            class: "nd-stepper {class}",
            role: "navigation",
            "aria-label": "Progress steps",

            for (index, step) in props.steps.iter().enumerate() {
                {
                    let status_label = match step.status {
                        StepStatus::Completed => "completed",
                        StepStatus::Current => "current",
                        StepStatus::Pending => "pending",
                    };
                    let aria_label = format!("Step {}, {}", index + 1, status_label);
                    let is_completed = step.status == StepStatus::Completed;
                    let is_pending = step.status == StepStatus::Pending;
                    let step_num = index + 1;
                    let label = step.label.clone();

                    rsx! {
                        // Step item
                        div {
                            class: "nd-step-item",
                            role: "listitem",

                            // Step circle
                            div {
                                class: if is_pending {
                                    "nd-step-circle nd-step-circle-pending"
                                } else {
                                    "nd-step-circle nd-step-circle-active"
                                },
                                "aria-label": aria_label,

                                if is_completed {
                                    span { class: "nd-step-check", "✓" }
                                } else {
                                    span { class: "nd-step-number", "{step_num}" }
                                }
                            }

                            // Step label
                            span {
                                class: "nd-step-label",
                                class: if is_pending { "nd-step-label-pending" } else { "" },
                                "{label}"
                            }
                        }

                        // Connector (between steps)
                        if index < total - 1 {
                            div {
                                class: if is_completed || step.status == StepStatus::Current {
                                    "nd-step-connector nd-step-connector-active"
                                } else {
                                    "nd-step-connector nd-step-connector-pending"
                                },
                                "aria-hidden": "true",
                            }
                        }
                    }
                }
            }
        }
    }
}
