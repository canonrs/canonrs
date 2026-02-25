use leptos::prelude::*;
use uuid::Uuid;
use rs_canonrs::application::Command;
use rs_canonrs::domain::{CanonNode, CanonBlockType};
use crate::ui::layout_builder::domain::node::{Node, NodeKind};
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use crate::ui::layout_builder::types::DragContext;
use super::super::state::drop_zone_types::DragVisualState;

pub fn handle_drop(
    ev: leptos::ev::PointerEvent,
    parent_id: Uuid,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    drag_visual: RwSignal<DragVisualState>,
) {
    ev.stop_propagation();
    let ctx = drag_ctx.get();
    if !ctx.is_dragging() { return; }
    let idx = drag_visual.get_untracked().insert_index;

    if let Some(src_id) = ctx.node_id {
        engine.update(|e| { let _ = e.execute(Command::Move { node_id: src_id, new_parent: parent_id, index: idx }); });
    } else if let Some(block) = ctx.block_def {
        let ok = tree.get_untracked().iter()
            .find(|n| n.id == parent_id)
            .map(|p: &Node| p.accepts(&block)).unwrap_or(true);
        if ok {
            let node_id = Uuid::new_v4();
            let block_type = CanonBlockType::from_id(block.id).expect("Unknown block type — registry mismatch");
            let mut canon = CanonNode::with_id(node_id, block_type);
            for r in block.regions {
                canon.add_child(CanonNode::new(CanonBlockType::Slot { name: r.id.to_string() }));
            }
            engine.update(|e| { let _ = e.execute(Command::Insert { parent_id, node: canon, index: idx }); });
        }
    } else if let Some(comp) = ctx.component_def {
        let ok = tree.get_untracked().iter()
            .find(|n| n.id == parent_id)
            .map(|p: &Node| p.accepts_component(&comp)).unwrap_or(true);
        if ok {
            let block_type = CanonBlockType::from_id(comp.id).expect("Unknown block type — registry mismatch");
            let canon = CanonNode::new(block_type);
            engine.update(|e| { let _ = e.execute(Command::Insert { parent_id, node: canon, index: idx }); });
        }
    }

    // Sincronizar engine → tree após comando
    let flat = engine.get_untracked().sync_flat();
    tree.set(flat);

    batch(move || {
        drag_visual.set(DragVisualState::empty());
        drag_ctx.set(DragContext::empty());
    });
}
