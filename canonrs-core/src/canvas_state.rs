//! CanvasState — tipos compartilhados entre SSR e hydrate

#[derive(Clone, Debug, Default)]
pub struct CanvasState {
    pub tool:     String,
    pub selected: Vec<u64>,
    pub state:    String,
}
