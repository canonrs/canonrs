//! Scheduler — RAF queue, observer batching, event coalescing
//!
//! Centralizes all async scheduling to prevent RAF storms,
//! observer storms, and duplicate work.

use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Priority levels for scheduled work
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Critical — runs immediately (sync)
    Critical = 0,
    /// High — next RAF
    High     = 1,
    /// Normal — batched RAF
    Normal   = 2,
    /// Low — idle
    Low      = 3,
}

struct ScheduledWork {
    key:      String,
    priority: Priority,
    work:     Box<dyn FnOnce()>,
}

thread_local! {
    static QUEUE:        RefCell<Vec<ScheduledWork>> = RefCell::new(vec![]);
    static RAF_PENDING:  RefCell<bool>               = RefCell::new(false);
    static COALESCED:    RefCell<HashMap<String, u32>> = RefCell::new(HashMap::new());
}

/// Schedule work with deduplication by key
/// If key already queued, replaces with new work (coalescing)
pub fn schedule<F>(key: &str, priority: Priority, work: F)
where F: FnOnce() + 'static
{
    QUEUE.with(|q| {
        let mut queue = q.borrow_mut();
        // Remove existing work with same key (coalesce)
        queue.retain(|w| w.key != key);
        queue.push(ScheduledWork {
            key:      key.to_string(),
            priority,
            work:     Box::new(work),
        });
        // Sort by priority
        queue.sort_by_key(|w| w.priority);
    });

    request_flush(priority);
}

/// Schedule RAF work — coalesced by key
pub fn raf<F>(key: &str, work: F)
where F: FnOnce() + 'static
{
    schedule(key, Priority::High, work);
}

/// Schedule batched work — coalesced, lower priority
pub fn batch<F>(key: &str, work: F)
where F: FnOnce() + 'static
{
    schedule(key, Priority::Normal, work);
}

fn request_flush(priority: Priority) {
    if priority == Priority::Critical {
        flush();
        return;
    }

    let already_pending = RAF_PENDING.with(|p| {
        if *p.borrow() { return true; }
        *p.borrow_mut() = true;
        false
    });

    if already_pending { return; }

    let cb = Closure::once(Box::new(|| {
        RAF_PENDING.with(|p| *p.borrow_mut() = false);
        flush();
    }) as Box<dyn FnOnce()>);

    if let Some(win) = web_sys::window() {
        let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
    }
    cb.forget();
}

/// Flush all pending work in priority order
pub fn flush() {
    let work: Vec<ScheduledWork> = QUEUE.with(|q| {
        let mut queue = q.borrow_mut();
        std::mem::take(&mut *queue)
    });
    for item in work {
        (item.work)();
    }
}

/// How many items are queued
pub fn pending_count() -> usize {
    QUEUE.with(|q| q.borrow().len())
}
