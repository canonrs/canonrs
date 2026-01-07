use leptos::prelude::*;

/// VirtualItem - Generic wrapper for any item type
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualItem<T> {
    /// Item index in source list
    pub index: usize,
    
    /// Actual data
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
    /// Height of each item in pixels
    pub item_height: f64,
    
    /// Height of viewport in pixels
    pub viewport_height: f64,
    
    /// Number of items to render outside viewport (buffer)
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
