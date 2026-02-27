#[derive(Clone, Debug, PartialEq)]
pub enum RegionOrientation { Row, Column }

#[derive(Clone, Debug, PartialEq)]
pub struct RegionRect {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DragVisualState {
    pub active_zone_id: Option<uuid::Uuid>,
    pub insert_index: usize,
    pub region_rect: Option<RegionRect>,
    pub orientation: RegionOrientation,
    pub insert_pos: f64, // top ou left do marker em px relativo à região
}

impl DragVisualState {
    pub fn empty() -> Self {
        Self {
            active_zone_id: None,
            insert_index: 0,
            region_rect: None,
            orientation: RegionOrientation::Column,
            insert_pos: 0.0,
        }
    }
}
