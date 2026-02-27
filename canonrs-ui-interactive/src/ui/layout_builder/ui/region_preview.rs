use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use crate::ui::layout_builder::types::{Node, DragContext, CanvasMode};
use super::drop_zone::DropZone;
use super::drop_zone::DragVisualState;

#[component]
pub fn RegionPreview(
    node: Node,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
) -> impl IntoView {
    let node_id = node.id;
    let is_builder = move || canvas_mode.get() == CanvasMode::Builder;
    let is_selected = move || selected_id.get() == Some(node_id);

    let label = match &node.kind {
        crate::ui::layout_builder::types::NodeKind::Region { label, .. } => label.clone(),
        _ => String::new(),
    };

    let label_for_drop = label.clone();
    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        selected_id.set(Some(node_id));
    };

    view! {
        <div
            data-region-preview=""
            style="display:contents;"
        >
            <DropZone
                parent_id=node_id
                engine=engine
                tree=tree
                drag_ctx=drag_ctx
                selected_id=selected_id
                canvas_mode=canvas_mode
                drag_visual=drag_visual
                slot_label=label_for_drop.clone()
            />
        </div>
    }
}
