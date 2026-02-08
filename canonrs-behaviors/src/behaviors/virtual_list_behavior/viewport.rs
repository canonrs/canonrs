use super::types::{ViewportRange, ScrollState, VirtualListConfig};

pub fn calculate_visible_range(
    scroll_state: ScrollState,
    total_items: usize,
    config: VirtualListConfig,
) -> ViewportRange {
    let scroll_top = scroll_state.scroll_top;
    let item_height = config.item_height;
    let viewport_height = config.viewport_height;
    let overscan = config.overscan;
    
    let start = ((scroll_top / item_height).floor() as usize)
        .saturating_sub(overscan);
    
    let visible_count = (viewport_height / item_height).ceil() as usize;
    let end = (start + visible_count + overscan * 2).min(total_items);
    
    ViewportRange { start, end }
}

pub fn calculate_total_height(total_items: usize, item_height: f64) -> f64 {
    total_items as f64 * item_height
}

pub fn calculate_offset(range: ViewportRange, item_height: f64) -> f64 {
    range.start as f64 * item_height
}
