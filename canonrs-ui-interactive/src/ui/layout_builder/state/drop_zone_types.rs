#[derive(Clone, Debug, PartialEq)]
pub struct DragVisualState {
    pub active_zone_id: Option<uuid::Uuid>,
    pub insert_index: usize,
}

impl DragVisualState {
    pub fn empty() -> Self { Self { active_zone_id: None, insert_index: 0 } }
}
