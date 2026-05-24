//! Ownership — component resource ownership tree
//!
//! Every resource (listener, timer, observer, RAF) is owned by a uid.
//! When a uid is destroyed, all its resources are freed atomically.
//! Supports parent-child ownership for subtree cleanup.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct OwnershipNode {
    /// Child uids owned by this node
    children: HashSet<String>,
    /// Parent uid (if any)
    parent: Option<String>,
    /// Resource counts for observability
    listener_count: usize,
    timer_count:    usize,
    observer_count: usize,
}

thread_local! {
    static TREE: RefCell<HashMap<String, OwnershipNode>> = RefCell::new(HashMap::new());
}

/// Register a uid in the ownership tree
pub fn register(uid: &str, parent_uid: Option<&str>) {
    TREE.with(|t| {
        let mut tree = t.borrow_mut();
        tree.entry(uid.to_string()).or_default().parent = parent_uid.map(|s| s.to_string());
        if let Some(parent) = parent_uid {
            tree.entry(parent.to_string()).or_default().children.insert(uid.to_string());
        }
    });
}

/// Track resource registration
pub fn track_listener(uid: &str) {
    TREE.with(|t| { t.borrow_mut().entry(uid.to_string()).or_default().listener_count += 1; });
}

pub fn track_timer(uid: &str) {
    TREE.with(|t| { t.borrow_mut().entry(uid.to_string()).or_default().timer_count += 1; });
}

pub fn track_observer(uid: &str) {
    TREE.with(|t| { t.borrow_mut().entry(uid.to_string()).or_default().observer_count += 1; });
}

/// Get all descendant uids (depth-first)
pub fn descendants(uid: &str) -> Vec<String> {
    TREE.with(|t| {
        let tree = t.borrow();
        let mut result = vec![];
        let mut stack = vec![uid.to_string()];
        while let Some(current) = stack.pop() {
            if let Some(node) = tree.get(&current) {
                for child in &node.children {
                    result.push(child.clone());
                    stack.push(child.clone());
                }
            }
        }
        result
    })
}

/// Remove uid and all descendants from ownership tree
/// Returns list of all uids that need cleanup
pub fn release(uid: &str) -> Vec<String> {
    let mut to_cleanup = descendants(uid);
    to_cleanup.push(uid.to_string());
    to_cleanup.reverse(); // children first

    TREE.with(|t| {
        let mut tree = t.borrow_mut();
        // Remove from parent
        if let Some(node) = tree.get(uid) {
            if let Some(parent) = node.parent.clone() {
                if let Some(parent_node) = tree.get_mut(&parent) {
                    parent_node.children.remove(uid);
                }
            }
        }
        for u in &to_cleanup {
            tree.remove(u);
        }
    });

    to_cleanup
}

/// Resource summary for observability
pub fn summary(uid: &str) -> Option<(usize, usize, usize)> {
    TREE.with(|t| {
        t.borrow().get(uid).map(|n| (n.listener_count, n.timer_count, n.observer_count))
    })
}

/// Total owned resources across all uids
pub fn total_resources() -> (usize, usize, usize) {
    TREE.with(|t| {
        let tree = t.borrow();
        tree.values().fold((0, 0, 0), |acc, n| {
            (acc.0 + n.listener_count, acc.1 + n.timer_count, acc.2 + n.observer_count)
        })
    })
}
