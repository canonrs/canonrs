//! CanvasState — tipos compartilhados entre SSR e hydrate

#[derive(Clone, Debug)]
pub struct CanvasState {
    pub tool:          String,
    pub selected:      Vec<u64>,
    pub state:         String,
    pub node_x:        f32,
    pub node_y:        f32,
    pub node_w:        f32,
    pub node_h:        f32,
    pub node_opacity:  f32,
    pub node_rotation: f32,
    pub node_fill:     String,
    pub node_stroke:       String,
    pub node_stroke_width: f32,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            tool:          String::new(),
            selected:      vec![],
            state:         String::new(),
            node_x:        0.0,
            node_y:        0.0,
            node_w:        0.0,
            node_h:        0.0,
            node_opacity:  1.0,
            node_rotation: 0.0,
            node_fill:     "#4a9eff".to_string(),
            node_stroke:       "".to_string(),
            node_stroke_width: 0.0,
        }
    }
}
