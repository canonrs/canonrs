use super::types::ViewportRange;

pub fn calculate_visible_range(
    scroll_top: f64,
    viewport_height: f64,
    row_height: f64,
    total_rows: usize,
    overscan: usize,
) -> ViewportRange {
    let visible_start = (scroll_top / row_height).floor() as usize;
    let visible_count = (viewport_height / row_height).ceil() as usize;
    
    let start = visible_start.saturating_sub(overscan);
    let end = (visible_start + visible_count + overscan).min(total_rows);
    
    ViewportRange { start, end }
}

pub fn calculate_total_height(total_rows: usize, row_height: f64) -> f64 {
    total_rows as f64 * row_height
}

pub fn calculate_offset(start_index: usize, row_height: f64) -> f64 {
    start_index as f64 * row_height
}
