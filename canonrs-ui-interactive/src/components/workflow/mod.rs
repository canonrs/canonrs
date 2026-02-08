pub mod types;
pub mod workflow;
pub mod step;
pub mod step_item;
pub mod audit;
pub mod audit_log;
pub mod rules;
pub mod gate;
pub mod demo;

// Re-export types
pub use types::{StepId, StepStatus, GateType, AuditEntry};

// Re-export main components
pub use workflow::{Workflow, WorkflowContext, use_workflow_context, transition_step};
pub use step::{Step, StepData};
pub use step_item::WorkflowStepItem;
pub use audit_log::{WorkflowAuditLog};
pub use demo::WorkflowDemo;
