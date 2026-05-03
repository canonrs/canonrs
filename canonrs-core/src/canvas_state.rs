//! CanvasState — UI state only
//! Engine is source of truth — never duplicate node data here

#[derive(Clone, Debug)]
pub struct CanvasState {
    pub selected: Vec<u64>,
    pub tool:     String,
    pub state:    String,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            selected: vec![],
            tool:     String::new(),
            state:    String::new(),
        }
    }
}
