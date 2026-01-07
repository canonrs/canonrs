use leptos::prelude::*;
use super::types::CommandRecord;

/// CommandHistory - Manages undo/redo stacks
#[derive(Clone, Copy)]
pub struct CommandHistory {
    /// Executed commands (undo stack)
    pub undo_stack: RwSignal<Vec<CommandRecord>>,
    
    /// Undone commands (redo stack)
    pub redo_stack: RwSignal<Vec<CommandRecord>>,
    
    /// Max history size
    pub max_size: usize,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            undo_stack: RwSignal::new(Vec::new()),
            redo_stack: RwSignal::new(Vec::new()),
            max_size,
        }
    }
    
    /// Push new command to history
    pub fn push(&self, record: CommandRecord) {
        self.undo_stack.update(|stack| {
            stack.push(record);
            
            // Limit size
            if stack.len() > self.max_size {
                stack.remove(0);
            }
        });
        
        // Clear redo stack when new command is executed
        self.redo_stack.update(|stack| stack.clear());
    }
    
    /// Undo last command
    pub fn undo(&self) -> Option<CommandRecord> {
        let mut record: Option<CommandRecord> = None;
        
        self.undo_stack.update(|stack| {
            record = stack.pop();
        });
        
        if let Some(ref cmd) = record {
            self.redo_stack.update(|stack| {
                stack.push(cmd.clone());
            });
        }
        
        record
    }
    
    /// Redo last undone command
    pub fn redo(&self) -> Option<CommandRecord> {
        let mut record: Option<CommandRecord> = None;
        
        self.redo_stack.update(|stack| {
            record = stack.pop();
        });
        
        if let Some(ref cmd) = record {
            self.undo_stack.update(|stack| {
                stack.push(cmd.clone());
            });
        }
        
        record
    }
    
    /// Can undo?
    pub fn can_undo(&self) -> Signal<bool> {
        let stack = self.undo_stack;
        Signal::derive(move || !stack.get().is_empty())
    }
    
    /// Can redo?
    pub fn can_redo(&self) -> Signal<bool> {
        let stack = self.redo_stack;
        Signal::derive(move || !stack.get().is_empty())
    }
    
    /// Clear all history
    pub fn clear(&self) {
        self.undo_stack.update(|s| s.clear());
        self.redo_stack.update(|s| s.clear());
    }
    
    /// Get undo stack size
    pub fn undo_count(&self) -> Signal<usize> {
        let stack = self.undo_stack;
        Signal::derive(move || stack.get().len())
    }
    
    /// Get redo stack size
    pub fn redo_count(&self) -> Signal<usize> {
        let stack = self.redo_stack;
        Signal::derive(move || stack.get().len())
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new(50) // Default: 50 commands
    }
}
