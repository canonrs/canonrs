/// Command - Represents an executable action in the system
/// 
/// **Type:** Pure data structure (no UI, no logic)
/// **Purpose:** Registry of available commands
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    /// Unique identifier (e.g., "workflow.complete_step")
    pub id: String,
    
    /// Human-readable label (e.g., "Complete Step")
    pub label: String,
    
    /// Optional group for organization (e.g., "Workflow", "Navigation")
    pub group: Option<String>,
    
    /// Optional keyboard shortcut (e.g., "Ctrl+Enter")
    pub shortcut: Option<String>,
    
    /// Optional icon
    pub icon: Option<String>,
    
    /// Callback to execute when command is triggered
    /// Stored as trait object to allow any callable
    pub callback: CommandCallback,
}

/// CommandCallback - Type-erased callback
#[derive(Clone)]
pub struct CommandCallback {
    inner: std::sync::Arc<dyn Fn() + Send + Sync>,
}

impl CommandCallback {
    pub fn new<F>(f: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static
    {
        Self {
            inner: std::sync::Arc::new(f),
        }
    }
    
    pub fn call(&self) {
        (self.inner)();
    }
}

impl std::fmt::Debug for CommandCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandCallback").finish()
    }
}

impl PartialEq for CommandCallback {
    fn eq(&self, _other: &Self) -> bool {
        // Callbacks can't be compared, always false
        false
    }
}

/// CommandGroup - Groups commands by category
#[derive(Clone, Debug, PartialEq)]
pub struct CommandGroup {
    pub id: String,
    pub label: String,
    pub commands: Vec<Command>,
}

impl CommandGroup {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            commands: Vec::new(),
        }
    }
    
    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }
}

/// CommandRegistry - Manages all available commands
#[derive(Clone, Debug, Default)]
pub struct CommandRegistry {
    groups: Vec<CommandGroup>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
        }
    }
    
    pub fn add_group(mut self, group: CommandGroup) -> Self {
        self.groups.push(group);
        self
    }
    
    pub fn get_all_commands(&self) -> Vec<Command> {
        self.groups
            .iter()
            .flat_map(|g| g.commands.iter())
            .cloned()
            .collect()
    }
    
    pub fn find_command(&self, id: &str) -> Option<Command> {
        self.get_all_commands()
            .into_iter()
            .find(|cmd| cmd.id == id)
    }
    
    pub fn search(&self, query: &str) -> Vec<Command> {
        let query_lower = query.to_lowercase();
        
        self.get_all_commands()
            .into_iter()
            .filter(|cmd| {
                cmd.label.to_lowercase().contains(&query_lower) ||
                cmd.id.to_lowercase().contains(&query_lower) ||
                cmd.group.as_ref().map_or(false, |g| g.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}
