//! AsyncBoundary Component
//! Unifica Loading + Error + Empty states para operações async


#[derive(Clone, Debug, PartialEq)]
pub enum AsyncState<T> {
    Loading,
    Empty,
    Error(String),
    Success(T),
}

impl<T> AsyncState<T> {
    pub fn is_loading(&self) -> bool {
        matches!(self, AsyncState::Loading)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, AsyncState::Empty)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, AsyncState::Error(_))
    }

    pub fn is_success(&self) -> bool {
        matches!(self, AsyncState::Success(_))
    }

    pub fn error_message(&self) -> Option<String> {
        match self {
            AsyncState::Error(msg) => Some(msg.clone()),
            _ => None,
        }
    }

    pub fn success_value(&self) -> Option<&T> {
        match self {
            AsyncState::Success(val) => Some(val),
            _ => None,
        }
    }
}

/// Helper component for async state management
/// Note: Due to Leptos limitations with Children in nested closures,
/// this component provides the AsyncState type and helper methods.
/// 
/// Usage pattern:
/// ```rust
/// let state = RwSignal::new(AsyncState::Loading);
/// 
/// view! {
///     {move || match state.get() {
///         AsyncState::Loading => view! { <LoadingOverlay /> }.into_any(),
///         AsyncState::Empty => view! { <EmptyTable /> }.into_any(),
///         AsyncState::Error(msg) => view! { <ErrorState message={msg} /> }.into_any(),
///         AsyncState::Success(data) => view! { /* render data */ }.into_any(),
///     }}
/// }
/// ```
pub struct AsyncBoundaryHelper;

impl AsyncBoundaryHelper {
    /// Create a loading state
    pub fn loading<T>() -> AsyncState<T> {
        AsyncState::Loading
    }

    /// Create an empty state
    pub fn empty<T>() -> AsyncState<T> {
        AsyncState::Empty
    }

    /// Create an error state
    pub fn error<T>(message: impl Into<String>) -> AsyncState<T> {
        AsyncState::Error(message.into())
    }

    /// Create a success state
    pub fn success<T>(value: T) -> AsyncState<T> {
        AsyncState::Success(value)
    }
}
