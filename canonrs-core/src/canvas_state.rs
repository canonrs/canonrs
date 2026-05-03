//! CanvasState — tipos compartilhados entre SSR e hydrate

#[derive(Clone, Debug, Default)]
pub struct CanvasState {
    pub tool:     String,
    pub selected: Vec<u64>,
    pub state:    String,
    pub node_x:       f32,
    pub node_y:       f32,
    pub node_w:       f32,
    pub node_h:       f32,
    pub node_opacity: f32,
}
