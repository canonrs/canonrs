use std::fmt::Debug;

/// Core command trait - all commands must implement this
pub trait Command: Debug + Send + Sync {
    /// Execute the command (mutate state)
    fn execute(&self);
    
    /// Undo the command (restore previous state)
    fn undo(&self);
    
    /// Human-readable description for UI
    fn description(&self) -> String;
    
    /// Can this command be undone?
    fn is_undoable(&self) -> bool {
        true
    }
}
