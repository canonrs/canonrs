use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct VirtualRow {
    pub index: usize,
    pub data: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct VirtualColumn {
    pub key: String,
    pub width: Option<usize>,
    pub flex: Option<f64>,
    pub align: ColumnAlign,
    pub resizable: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum ColumnAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollState {
    pub scroll_top: f64,
    pub scroll_left: f64,
}
