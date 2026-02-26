use leptos::prelude::*;
use std::sync::OnceLock;
use rs_canonrs::domain::CanonDocument;
use crate::ui::theme_engine::ThemeState;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use crate::ui::layout_builder::types::{DragContext, CanvasMode, ActiveLayout, init_slots};
use crate::ui::layout_builder::state::drop_zone_types::DragVisualState;
use crate::ui::layout_builder::domain::node::Node;
use uuid::Uuid;
use rs_canonrs::domain::CanonNode;
use rs_canonrs::domain::CanonBlockType;

static APP_OWNER: OnceLock<Owner> = OnceLock::new();
static THEME_STATE: OnceLock<ThemeState> = OnceLock::new();
static BUILDER_ENGINE:   OnceLock<RwSignal<BuilderEngine>>   = OnceLock::new();
static BUILDER_TREE:     OnceLock<RwSignal<Vec<Node>>>       = OnceLock::new();
static BUILDER_SELECTED: OnceLock<RwSignal<Option<Uuid>>>    = OnceLock::new();
static BUILDER_DIRTY:    OnceLock<RwSignal<bool>>            = OnceLock::new();
static BUILDER_LAYOUT:   OnceLock<RwSignal<ActiveLayout>>    = OnceLock::new();
static BUILDER_SLOTS:    OnceLock<RwSignal<Vec<Node>>>       = OnceLock::new();
static BUILDER_DRAG_CTX: OnceLock<RwSignal<DragContext>>     = OnceLock::new();
static BUILDER_CANVAS:   OnceLock<RwSignal<CanvasMode>>      = OnceLock::new();
static BUILDER_DRAG_VIS: OnceLock<RwSignal<DragVisualState>> = OnceLock::new();
static WORKSPACE_MODE:   OnceLock<RwSignal<WorkspaceMode>>   = OnceLock::new();

#[derive(Clone, Copy, PartialEq)]
pub enum WorkspaceMode { Builder, Theme }

pub fn app_owner() -> &'static Owner {
    APP_OWNER.get_or_init(Owner::new)
}

pub fn global_theme() -> ThemeState {
    THEME_STATE.get_or_init(|| ThemeState::new(app_owner())).clone()
}

pub fn global_workspace_mode() -> RwSignal<WorkspaceMode> {
    *WORKSPACE_MODE.get_or_init(|| app_owner().with(|| RwSignal::new(WorkspaceMode::Builder)))
}
pub fn global_engine() -> RwSignal<BuilderEngine> {
    *BUILDER_ENGINE.get_or_init(|| app_owner().with(|| RwSignal::new(BuilderEngine::new(CanonDocument::new("dashboard")))))
}
pub fn global_tree() -> RwSignal<Vec<Node>> {
    *BUILDER_TREE.get_or_init(|| app_owner().with(|| RwSignal::new(vec![])))
}
pub fn global_selected() -> RwSignal<Option<Uuid>> {
    *BUILDER_SELECTED.get_or_init(|| app_owner().with(|| RwSignal::new(None)))
}
pub fn global_dirty() -> RwSignal<bool> {
    *BUILDER_DIRTY.get_or_init(|| app_owner().with(|| RwSignal::new(false)))
}
pub fn global_active_layout() -> RwSignal<ActiveLayout> {
    *BUILDER_LAYOUT.get_or_init(|| app_owner().with(|| RwSignal::new(ActiveLayout::Dashboard)))
}
pub fn global_slots() -> RwSignal<Vec<Node>> {
    *BUILDER_SLOTS.get_or_init(|| app_owner().with(|| RwSignal::new(init_slots(&ActiveLayout::Dashboard))))
}
pub fn global_drag_ctx() -> RwSignal<DragContext> {
    *BUILDER_DRAG_CTX.get_or_init(|| app_owner().with(|| RwSignal::new(DragContext::empty())))
}
pub fn global_canvas_mode() -> RwSignal<CanvasMode> {
    *BUILDER_CANVAS.get_or_init(|| app_owner().with(|| RwSignal::new(CanvasMode::Builder)))
}
pub fn global_drag_visual() -> RwSignal<DragVisualState> {
    *BUILDER_DRAG_VIS.get_or_init(|| app_owner().with(|| RwSignal::new(DragVisualState::empty())))
}

/// Bootstrap sem layout node — apenas prepara engine com slots para receber drop
pub fn bootstrap_engine_with_layout(layout: &crate::ui::layout_builder::types::ActiveLayout) {
    use crate::ui::layout_builder::domain::node::Node;

    let engine = global_engine();
    let slots_signal = global_slots();
    let tree = global_tree();

    // Slots soltos (sem parent) — aguardando layout ser dropado
    let slot_nodes: Vec<Node> = layout.slots().into_iter()
        .map(|name| Node::slot(name))
        .collect();

    let mut doc = CanonDocument::new(layout.id().to_string());
    for slot in &slot_nodes {
        doc.nodes.push(CanonNode::with_id(
            slot.id,
            CanonBlockType::Slot { name: slot.label().to_string() },
        ));
    }

    engine.update(|e| *e = BuilderEngine::new(doc));
    slots_signal.set(slot_nodes);
    tree.set(vec![]);
}

/// Chamado pelo drop_handler quando layout é dropado — cria nó Layout como root
pub fn drop_layout(layout: &crate::ui::layout_builder::types::ActiveLayout) {
    use crate::ui::layout_builder::domain::node::{Node, NodeKind};

    let engine = global_engine();
    let slots_signal = global_slots();
    let tree = global_tree();

    let layout_node = Node::layout(layout);
    let layout_id = layout_node.id;

    let slot_nodes: Vec<Node> = layout.slots().into_iter()
        .map(|name| {
            let mut s = Node::slot(name);
            s.parent_id = Some(layout_id);
            s
        })
        .collect();

    let mut layout_canon = CanonNode::with_id(layout_id, CanonBlockType::Layout);
    for slot in &slot_nodes {
        layout_canon.add_child(CanonNode::with_id(
            slot.id,
            CanonBlockType::Slot { name: slot.label().to_string() },
        ));
    }
    let mut doc = CanonDocument::new(layout.id().to_string());
    doc.nodes.push(layout_canon);

    let mut flat_tree = vec![layout_node];
    flat_tree.extend(slot_nodes.clone());

    engine.update(|e| *e = BuilderEngine::new(doc));
    slots_signal.set(slot_nodes);
    tree.set(flat_tree);
}

pub fn init_global_pointer_listeners() {
    use crate::ui::layout_builder::state::drop_zone_types::DragVisualState;
    use crate::ui::layout_builder::types::DragContext;
    use crate::ui::layout_builder::ui::drop_handler::handle_drop;

    let drag_ctx   = global_drag_ctx();
    let drag_visual = global_drag_visual();
    let engine     = global_engine();
    let tree       = global_tree();

    crate::infra::web_pointer::register_pointerup_listener(move |ev| {
        use leptos::wasm_bindgen::JsCast;
        if !drag_ctx.get_untracked().is_dragging() { return; }
        let mut target_zone: Option<uuid::Uuid> = None;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.element_from_point(ev.client_x() as f32, ev.client_y() as f32) {
                    let mut cur: web_sys::Element = el;
                    loop {
                        if cur.has_attribute("data-drop-zone") {
                            if let Some(id_str) = cur.get_attribute("data-zone-id") {
                                if let Ok(uuid) = uuid::Uuid::parse_str(&id_str) {
                                    target_zone = Some(uuid);
                                }
                            }
                            break;
                        }
                        match cur.parent_element() { Some(p) => cur = p, None => break }
                    }
                }
            }
        }
        if let Some(parent_id) = target_zone {
            handle_drop(ev, parent_id, engine, tree, drag_ctx, drag_visual);
        } else {
            batch(move || {
                drag_visual.set(DragVisualState::empty());
                drag_ctx.set(DragContext::empty());
            });
        }
    });

    let drag_ctx2   = global_drag_ctx();
    let drag_visual2 = global_drag_visual();

    crate::infra::web_pointer::register_pointer_listener(move |ev| {
        use leptos::wasm_bindgen::JsCast;
        if !drag_ctx2.get_untracked().is_dragging() { return; }
        let mut new_active: Option<web_sys::Element> = None;
        if let Some(target) = ev.target() {
            let mut el: web_sys::Element = target.unchecked_into();
            loop {
                if el.has_attribute("data-drop-zone") { new_active = Some(el); break; }
                match el.parent_element() { Some(p) => el = p, None => break }
            }
        }
        if let Some(el) = new_active {
            let _ = el.set_attribute("data-drop-zone-active", "true");
            let client_y = ev.client_y() as f64;
            let mut idx = 0usize;
            if let Ok(blocks) = el.query_selector_all(":scope > [data-block-preview]") {
                let len = blocks.length() as usize; idx = len;
                let mut vi = 0usize;
                for i in 0..len {
                    if let Some(b) = blocks.item(i as u32) {
                        let be: web_sys::Element = b.unchecked_into();
                        if be.get_attribute("data-dragging").as_deref() == Some("true") { continue; }
                        let rect = be.get_bounding_client_rect();
                        if client_y < rect.top() + rect.height() / 2.0 { idx = vi; break; }
                        vi += 1; idx = vi;
                    }
                }
            }
            if let Some(zone_id_str) = el.get_attribute("data-zone-id") {
                if let Ok(zone_uuid) = uuid::Uuid::parse_str(&zone_id_str) {
                    drag_visual2.set(DragVisualState { active_zone_id: Some(zone_uuid), insert_index: idx });
                }
            }
        }
    });
}
