//! DOM utility helpers for WASM behaviors

use wasm_bindgen::JsCast;
use web_sys::{NodeList, Node};

/// Iterates a `NodeList`, casting each item to `T`.
/// Skips items that fail the cast.
pub fn node_list_iter<T: JsCast>(list: &NodeList) -> impl Iterator<Item = T> + '_ {
    (0..list.length()).filter_map(|i| {
        list.item(i).and_then(|n: Node| n.dyn_into::<T>().ok())
    })
}
