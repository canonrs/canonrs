//! Drag — state management via DOM attributes (GC-safe, no Element capture)
use std::cell::Cell;
use std::rc::Rc;
use web_sys::Element;

// --- DOM-based drag state (para resizable e futuros drag components) ---

pub fn set_drag(root: &Element, ptr_id: i32, size: f64, offset: f64) {
    let _ = root.set_attribute("data-rs-drag-ptr",    &ptr_id.to_string());
    let _ = root.set_attribute("data-rs-drag-size",   &size.to_string());
    let _ = root.set_attribute("data-rs-drag-offset", &offset.to_string());
}

pub fn clear_drag(root: &Element) {
    let _ = root.remove_attribute("data-rs-drag-ptr");
    let _ = root.remove_attribute("data-rs-drag-size");
    let _ = root.remove_attribute("data-rs-drag-offset");
}

pub fn drag_active(root: &Element, ptr_id: i32) -> bool {
    root.get_attribute("data-rs-drag-ptr")
        .and_then(|s| s.parse::<i32>().ok())
        .map(|p| p == ptr_id)
        .unwrap_or(false)
}

pub fn drag_size(root: &Element) -> f64 {
    root.get_attribute("data-rs-drag-size").and_then(|s| s.parse().ok()).unwrap_or(0.0)
}

pub fn drag_offset(root: &Element) -> f64 {
    root.get_attribute("data-rs-drag-offset").and_then(|s| s.parse().ok()).unwrap_or(0.0)
}

// --- DragState (Cell-based, para slider e scroll_area) ---

pub type DragState = Rc<Cell<Option<u32>>>;

pub fn new() -> DragState { Rc::new(Cell::new(None)) }
pub fn start(state: &DragState, ptr_id: i32) { state.set(Some(ptr_id as u32)); }
pub fn stop(state: &DragState) { state.set(None); }
pub fn is_active(state: &DragState, ptr_id: i32) -> bool {
    matches!(state.get(), Some(id) if id == ptr_id as u32)
}

// --- DragContext (Cell-based com size/offset, para componentes que precisam de mais estado) ---

pub struct DragContext {
    pub ptr_id: Cell<Option<u32>>,
    pub size:   Cell<f64>,
    pub offset: Cell<f64>,
}

impl DragContext {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            ptr_id: Cell::new(None),
            size:   Cell::new(0.0),
            offset: Cell::new(0.0),
        })
    }

    pub fn start(&self, ptr_id: i32, size: f64, offset: f64) {
        self.ptr_id.set(Some(ptr_id as u32));
        self.size.set(size);
        self.offset.set(offset);
    }

    pub fn stop(&self) {
        self.ptr_id.set(None);
    }

    pub fn is_active(&self, ptr_id: i32) -> bool {
        matches!(self.ptr_id.get(), Some(id) if id == ptr_id as u32)
    }

    pub fn is_dragging(&self) -> bool {
        self.ptr_id.get().is_some()
    }
}
