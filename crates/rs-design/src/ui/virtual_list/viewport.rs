use super::types::{ViewportRange, ScrollState, VirtualListConfig};

/// Calculate visible range based on scroll position
pub fn calculate_visible_range(
    scroll_state: ScrollState,
    total_items: usize,
    config: VirtualListConfig,
) -> ViewportRange {
    let scroll_top = scroll_state.scroll_top;
    let item_height = config.item_height;
    let viewport_height = config.viewport_height;
    let overscan = config.overscan;

    // Start index (with overscan buffer)
    let start = ((scroll_top / item_height).floor() as usize)
        .saturating_sub(overscan);

    // Visible count
    let visible_count = (viewport_height / item_height).ceil() as usize;

    // End index (with overscan buffer)
    let end = (start + visible_count + overscan * 2).min(total_items);

    ViewportRange { start, end }
}

/// Calculate total content height
pub fn calculate_total_height(total_items: usize, item_height: f64) -> f64 {
    total_items as f64 * item_height
}

/// Calculate offset for visible range
pub fn calculate_offset(range: ViewportRange, item_height: f64) -> f64 {
    range.start as f64 * item_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_visible_range() {
        let config = VirtualListConfig {
            item_height: 40.0,
            viewport_height: 400.0,
            overscan: 2,
        };

        let scroll_state = ScrollState { scroll_top: 0.0 };
        let range = calculate_visible_range(scroll_state, 100, config);

        assert_eq!(range.start, 0);
        assert_eq!(range.end, 14); // 10 visible + 4 overscan
    }

    #[test]
    fn test_total_height() {
        let height = calculate_total_height(1000, 40.0);
        assert_eq!(height, 40000.0);
    }
}
