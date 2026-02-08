#[derive(Clone, Debug, PartialEq)]
pub struct VirtualItem<T> {
    pub index: usize,
    pub data: T,
}

impl<T> VirtualItem<T> {
    pub fn new(index: usize, data: T) -> Self {
        Self { index, data }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScrollState {
    pub scroll_top: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VirtualListConfig {
    pub item_height: f64,
    pub viewport_height: f64,
    pub overscan: usize,
}

impl Default for VirtualListConfig {
    fn default() -> Self {
        Self {
            item_height: 36.0,
            viewport_height: 600.0,
            overscan: 5,
        }
    }
}
