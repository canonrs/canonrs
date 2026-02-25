use leptos::prelude::*;
use crate::ui::layout_builder::types::Node;

pub const VIRT_THRESHOLD: usize = 30;
pub const ESTIMATED_HEIGHT: f64 = 72.0;

pub struct VirtState {
    pub scroll_top: RwSignal<f64>,
    pub container_height: RwSignal<f64>,
    pub item_heights: RwSignal<Vec<f64>>,
}

impl VirtState {
    pub fn new() -> Self {
        Self {
            scroll_top: RwSignal::new(0.0),
            container_height: RwSignal::new(600.0),
            item_heights: RwSignal::new(vec![]),
        }
    }
}

pub fn compute_offsets(heights: &[f64], n: usize) -> Vec<f64> {
    let mut acc = vec![0.0f64; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + heights.get(i).copied().unwrap_or(ESTIMATED_HEIGHT);
    }
    acc
}

pub fn compute_visible_range(st: f64, ch: f64, acc: &[f64], n: usize) -> (usize, usize) {
    if n == 0 { return (0, 0); }
    let mut lo = 0; let mut hi = n;
    while lo < hi {
        let m = (lo + hi) / 2;
        if acc[m + 1] < st { lo = m + 1; } else { hi = m; }
    }
    let start = lo.saturating_sub(2);
    let mut end = start;
    while end < n && acc[end] < st + ch + ESTIMATED_HEIGHT * 2.0 { end += 1; }
    (start, (end + 2).min(n))
}
