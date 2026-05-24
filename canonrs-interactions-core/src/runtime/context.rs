//! RuntimeContext — typed context passed to component mount functions
//!
//! Replaces raw Element passing with a governed context object.
//! Provides access to all runtime services with uid binding.

use web_sys::Element;
use super::lifecycle::LifecycleState;

/// Runtime context passed to component mount/init functions
/// Binds all runtime services to the component's uid
pub struct RuntimeContext {
    /// Component root element
    pub root: Element,
    /// Component uid (data-rs-uid)
    pub uid:  String,
    /// Interaction group
    pub group: String,
}

impl RuntimeContext {
    pub fn new(root: Element, uid: String, group: String) -> Self {
        Self { root, uid, group }
    }

    /// Try to create from element — returns None if uid missing
    pub fn from_element(el: Element) -> Option<Self> {
        let uid   = el.get_attribute("data-rs-uid")?;
        let group = el.get_attribute("data-rs-interaction").unwrap_or_default();
        Some(Self::new(el, uid, group))
    }

    /// Register a listener owned by this component
    pub fn listen<F>(&self, event: &str, cb: F) -> usize
    where F: FnMut(web_sys::Event) + 'static
    {
        let id = super::listeners::listen(&self.uid, &self.root, event, cb);
        super::ownership::track_listener(&self.uid);
        id
    }

    /// Register a document listener owned by this component
    pub fn listen_document<F>(&self, event: &str, cb: F) -> usize
    where F: FnMut(web_sys::Event) + 'static
    {
        let id = super::listeners::listen_document(&self.uid, event, cb);
        super::ownership::track_listener(&self.uid);
        id
    }

    /// Register a window listener owned by this component
    pub fn listen_window<F>(&self, event: &str, cb: F) -> usize
    where F: FnMut(web_sys::Event) + 'static
    {
        let id = super::listeners::listen_window(&self.uid, event, cb);
        super::ownership::track_listener(&self.uid);
        id
    }

    /// Schedule RAF work coalesced by key
    pub fn raf<F>(&self, key: &str, work: F)
    where F: FnOnce() + 'static
    {
        let raf_key = format!("{}:{}", self.uid, key);
        super::scheduler::raf(&raf_key, work);
    }

    /// Schedule batched work
    pub fn batch<F>(&self, key: &str, work: F)
    where F: FnOnce() + 'static
    {
        let batch_key = format!("{}:{}", self.uid, key);
        super::scheduler::batch(&batch_key, work);
    }

    /// Cleanup all resources owned by this component
    pub fn cleanup(&self) {
        super::cleanup::cleanup_uid(&self.uid);
        super::lifecycle::set_state(&self.uid, LifecycleState::Destroy);
        super::ownership::release(&self.uid);
    }

    /// Transition lifecycle state
    pub fn transition(&self, state: LifecycleState) -> bool {
        super::lifecycle::transition(&self.uid, state)
    }

    /// Current lifecycle state
    pub fn state(&self) -> Option<LifecycleState> {
        super::lifecycle::state(&self.uid)
    }
}
