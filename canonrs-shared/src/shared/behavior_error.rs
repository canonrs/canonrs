//! Error types for behavior system
use std::fmt;

#[derive(Debug, Clone)]
pub enum BehaviorError {
    ElementNotFound { selector: String },
    InvalidAttribute { attr: String, value: String },
    EventListenerFailed { element_id: String, event: String },
    ObserverFailed { reason: String },
    JsError { message: String },
}

impl fmt::Display for BehaviorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElementNotFound { selector } => 
                write!(f, "Element not found: {}", selector),
            Self::InvalidAttribute { attr, value } => 
                write!(f, "Invalid attribute '{}': {}", attr, value),
            Self::EventListenerFailed { element_id, event } => 
                write!(f, "Failed to attach '{}' to element '{}'", event, element_id),
            Self::ObserverFailed { reason } => 
                write!(f, "MutationObserver failed: {}", reason),
            Self::JsError { message } => 
                write!(f, "JS error: {}", message),
        }
    }
}

impl std::error::Error for BehaviorError {}

pub type BehaviorResult<T> = Result<T, BehaviorError>;
