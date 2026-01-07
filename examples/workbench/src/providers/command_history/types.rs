use serde::{Serialize, Deserialize};

/// Command - Representa uma ação executável e reversível
pub trait Command: Send + Sync {
    /// Execute the command
    fn execute(&self);
    
    /// Undo the command
    fn undo(&self);
    
    /// Get command description for UI
    fn description(&self) -> String;
}

/// CommandRecord - Registro de comando executado
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandRecord {
    pub id: String,
    pub description: String,
    pub timestamp: i64,
    pub command_type: String,
    pub data: String, // JSON serialized command data
}

impl CommandRecord {
    pub fn new(description: String, command_type: String, data: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        Self {
            id: format!("cmd-{}", timestamp),
            description,
            timestamp,
            command_type,
            data,
        }
    }
}
