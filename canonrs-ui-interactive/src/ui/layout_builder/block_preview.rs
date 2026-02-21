use leptos::prelude::*;
use canonrs_ui::blocks::{Card, AlertBlock, AlertVariant};
use super::types::{Node, NodeKind, DragContext, remove_node};
use super::drop_zone::DropZone;

#[component]
pub fn BlockPreview(
    node: Node,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
) -> impl IntoView {
    let node_id = node.id;
    let parent_id = node.parent_id;
    let is_container = node.is_container();
    let is_dragging_this = move || drag_ctx.get().node_id == Some(node_id);

    let (block_id, block_label, block_icon) = match &node.kind {
        NodeKind::Block { def } => (def.id, def.label, def.icon),
        NodeKind::Slot { name } => (*name, *name, "▤"),
    };

    let block_def = match node.kind.clone() {
        NodeKind::Block { def } => Some(def),
        _ => None,
    };

    let remove = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        tree.update(|t| remove_node(t, node_id));
    };

    let on_pointerdown = move |ev: leptos::ev::PointerEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        if let Some(def) = block_def.clone() {
            drag_ctx.set(DragContext {
                node_id: Some(node_id),
                block_def: Some(def),
            });
        }
    };

    let inner = match block_id {
        "card" => view! {
            <Card>
                <div style="padding: 0.5rem; font-size: 0.8rem; color: var(--theme-surface-fg-muted);">"Card content"</div>
            </Card>
        }.into_any(),
        "alert" => view! {
            <AlertBlock variant=AlertVariant::Info>"Alert message"</AlertBlock>
        }.into_any(),
        _ => view! {
            <div data-block-inner="">
                <span>{block_icon}</span>
                <span>{block_label}</span>
            </div>
        }.into_any(),
    };

    view! {
        <div
            data-block-preview=""
            attr:data-dragging=move || if is_dragging_this() { "true" } else { "false" }
            on:pointerdown=on_pointerdown
        >
            {inner}
            {move || if is_container {
                Some(view! {
                    <div data-block-children="" on:pointermove=move |ev| ev.stop_propagation()>
                        <DropZone parent_id=node_id tree=tree drag_ctx=drag_ctx slot_label="children" />
                    </div>
                }.into_any())
            } else { None }}
            <button
                data-block-remove=""
                on:click=remove
                on:pointerdown=move |ev| ev.stop_propagation()
            >
                "×"
            </button>
        </div>
    }
}
