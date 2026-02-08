use super::command_trait::Command;
use std::sync::Arc;
use std::fmt::Debug;

/// MacroCommand - Executes multiple commands as one atomic operation
#[derive(Clone)]
pub struct MacroCommand {
    pub commands: Vec<Arc<dyn Command>>,
    pub description: String,
}

impl Debug for MacroCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MacroCommand")
            .field("description", &self.description)
            .field("command_count", &self.commands.len())
            .finish()
    }
}

impl MacroCommand {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            commands: Vec::new(),
            description: description.into(),
        }
    }
    
    pub fn add(mut self, command: impl Command + 'static) -> Self {
        self.commands.push(Arc::new(command));
        self
    }
    
    pub fn from_vec(commands: Vec<Arc<dyn Command>>, description: impl Into<String>) -> Self {
        Self {
            commands,
            description: description.into(),
        }
    }
}

impl Command for MacroCommand {
    fn execute(&self) {
        leptos::logging::log!("🎬 Executing macro: {}", self.description);
        for (i, cmd) in self.commands.iter().enumerate() {
            leptos::logging::log!("  {}. {}", i + 1, cmd.description());
            cmd.execute();
        }
    }
    
    fn undo(&self) {
        leptos::logging::log!("⏪ Undoing macro: {}", self.description);
        // Undo in reverse order
        for (i, cmd) in self.commands.iter().enumerate().rev() {
            leptos::logging::log!("  {}. Undo {}", i + 1, cmd.description());
            cmd.undo();
        }
    }
    
    fn description(&self) -> String {
        self.description.clone()
    }
}
