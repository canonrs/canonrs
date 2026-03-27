//! Error and Result types shared across all behaviors

#[derive(Debug, Clone)]
pub enum BehaviorError {
    JsError { message: String },
    ElementNotFound { selector: String },
    ObserverFailed { reason: String },
}

impl std::fmt::Display for BehaviorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BehaviorError::JsError { message } => write!(f, "JsError: {}", message),
            BehaviorError::ElementNotFound { selector } => write!(f, "ElementNotFound: {}", selector),
            BehaviorError::ObserverFailed { reason } => write!(f, "ObserverFailed: {}", reason),
        }
    }
}

impl std::error::Error for BehaviorError {}

pub type BehaviorResult<T = ()> = Result<T, BehaviorError>;
