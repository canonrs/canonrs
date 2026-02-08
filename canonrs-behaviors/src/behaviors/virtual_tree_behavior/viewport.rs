use super::types::{ViewportRange, ScrollState};

pub fn calculate_visible_range(
    scroll_state: ScrollState,
    total_rows: usize,
    row_height: f64,
    viewport_height: f64,
    overscan: usize,
) -> ViewportRange {
    let scroll_top = scroll_state.scroll_top;
    let start = ((scroll_top / row_height).floor() as usize).saturating_sub(overscan);
    let visible_count = (viewport_height / row_height).ceil() as usize;
    let end = (start + visible_count + overscan * 2).min(total_rows);
    
    ViewportRange { start, end }
}

pub fn calculate_total_height(total_rows: usize, row_height: f64) -> f64 {
    total_rows as f64 * row_height
}

pub fn calculate_offset(range: ViewportRange, row_height: f64) -> f64 {
    range.start as f64 * row_height
}
