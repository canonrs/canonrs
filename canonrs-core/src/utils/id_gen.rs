//! Geração de IDs determinística — mesmo resultado no SSR e no hydrate.

use std::sync::atomic::{AtomicUsize, Ordering};

static TOGGLE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static ACCORDION_ROOT_COUNTER: AtomicUsize = AtomicUsize::new(0);
static ACCORDION_ITEM_COUNTER: AtomicUsize = AtomicUsize::new(0);
static COLLAPSIBLE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn gen_toggle_id() -> String {
    format!("toggle-{}", TOGGLE_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn gen_accordion_id() -> String {
    format!("accordion-{}", ACCORDION_ROOT_COUNTER.fetch_add(1, Ordering::SeqCst))
}

/// Gera par de ids para trigger e content dentro de um AccordionItem.
/// Contador próprio — não interfere com gen_accordion_id().
pub fn gen_accordion_item_ids() -> (String, String) {
    let n = ACCORDION_ITEM_COUNTER.fetch_add(1, Ordering::SeqCst);
    (format!("acc-trigger-{}", n), format!("acc-content-{}", n))
}

pub fn gen_collapsible_id() -> String {
    format!("collapsible-{}", COLLAPSIBLE_COUNTER.fetch_add(1, Ordering::SeqCst))
}
