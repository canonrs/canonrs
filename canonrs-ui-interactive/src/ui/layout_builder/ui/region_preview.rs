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
            attr:data-selected=move || if is_selected() { "true" } else { "false" }
            on:click=on_click
            on:pointerdown=move |ev| ev.stop_propagation()
            on:pointerup=move |ev| ev.stop_propagation()
            style=move || if is_selected() {
                "margin-top: 0.25rem; border-radius: var(--builder-block-radius); outline: 2px solid var(--theme-action-primary-bg); outline-offset: 1px;"
            } else {
                "margin-top: 0.25rem;"
            }
        >
            {move || if is_builder() {
                let label_clone = label.clone();
                Some(view! {
                    <div style="font-size: 0.6rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--theme-surface-fg-muted); padding: 0.1rem 0.25rem; margin-bottom: 0.1rem;">
                        {label_clone}
                    </div>
                })
            } else { None }}
            <div
                on:pointermove=move |ev| ev.stop_propagation()
                style=move || if drag_ctx.get().is_dragging() && is_builder() {
                    "border: 2px solid var(--builder-dropzone-active-border); border-radius: var(--builder-block-radius); background: var(--builder-dropzone-hover-bg); padding: 0.25rem;"
                } else { "" }
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
        </div>
    }
}
