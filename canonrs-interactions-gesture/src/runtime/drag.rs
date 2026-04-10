//! Drag — pointer state via Cell exclusivamente (zero RefCell, zero panic)
use std::cell::Cell;
use std::rc::Rc;

// DragContext — estado de drag sem nenhum Element capturado
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

// DragState — API simples para slider e scroll_area
pub type DragState = Rc<Cell<Option<u32>>>;

pub fn new() -> DragState { Rc::new(Cell::new(None)) }
pub fn start(state: &DragState, ptr_id: i32) { state.set(Some(ptr_id as u32)); }
pub fn stop(state: &DragState) { state.set(None); }
pub fn is_active(state: &DragState, ptr_id: i32) -> bool {
    matches!(state.get(), Some(id) if id == ptr_id as u32)
}
