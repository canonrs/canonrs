use leptos::prelude::*;
use super::types::{Node, NodeKind, BlockDef, DragContext, children_of, insert_node, remove_node};
use super::block_preview::BlockPreview;

#[component]
pub fn DropZone(
    /// ID do nó pai (Slot ou Block container)
    parent_id: uuid::Uuid,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    #[prop(optional)] slot_label: Option<&'static str>,
) -> impl IntoView {
    let hover = RwSignal::new(false);
    let insert_index: RwSignal<usize> = RwSignal::new(0);
    let is_dragging = move || drag_ctx.get().is_dragging();
    let get_children = move || children_of(&tree.get(), parent_id);

    let handle_pointermove = move |ev: leptos::ev::PointerEvent| {
        if !is_dragging() { return; }
        hover.set(true);
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;
            let client_y = ev.client_y() as f64;
            if let Some(target) = ev.current_target() {
                let el: web_sys::Element = target.unchecked_into();
                let rect = el.get_bounding_client_rect();
                let rel_y = (client_y - rect.top()).max(0.0);
                let zone_h = rect.height().max(1.0);
                let ratio = (rel_y / zone_h).clamp(0.0, 1.0);
                let len = get_children().len();
                let idx = ((ratio * len as f64).round() as usize).min(len);
                insert_index.set(idx);
            }
        }
    };

    let handle_pointerup = move |ev: leptos::ev::PointerEvent| {
        ev.stop_propagation();
        let ctx = drag_ctx.get();
        if !ctx.is_dragging() || !hover.get() { return; }
        let block = match ctx.block_def { Some(b) => b, None => return };

        // Constraint Engine: valida se este container aceita o bloco
        let parent_node = tree.get().iter().find(|n| n.id == parent_id).cloned();
        if let Some(ref p) = parent_node {
            if !p.accepts(&block) {
                hover.set(false);
                return;
            }
        }

        if let Some(src_id) = ctx.node_id {
            tree.update(|t| remove_node(t, src_id));
        }

        let idx = insert_index.get();
        let node = Node::block(block, parent_id, idx);
        tree.update(|t| insert_node(t, node));

        hover.set(false);
        drag_ctx.set(DragContext::empty());
    };

    let handle_pointerleave = move |_: leptos::ev::PointerEvent| {
        hover.set(false);
    };

    let label = slot_label.unwrap_or("drop");

    view! {
        <div
            data-drop-zone=""
            attr:data-hover=move || if hover.get() { "true" } else { "false" }
            attr:data-dragging=move || if is_dragging() { "true" } else { "false" }
            on:pointermove=handle_pointermove
            on:pointerup=handle_pointerup
            on:pointerleave=handle_pointerleave
        >
            {move || {
                let nodes = get_children();
                let idx = insert_index.get();
                let hovering = hover.get();

                if nodes.is_empty() {
                    return view! {
                        <div data-drop-zone-empty="">
                            {format!("Drop here → {}", label)}
                        </div>
                    }.into_any();
                }

                nodes.into_iter().enumerate().flat_map(|(i, n)| {
                    let line = if hovering && idx == i {
                        Some(view! {
                            <div style="height: 3px; background: #6366f1; border-radius: 2px; margin: 2px 0; pointer-events: none;" />
                        }.into_any())
                    } else { None };
                    vec![
                        line,
                        Some(view! {
                            <BlockPreview node=n tree=tree drag_ctx=drag_ctx />
                        }.into_any()),
                    ].into_iter().flatten()
                }).chain(if hovering && idx >= get_children().len() {
                    Some(view! {
                        <div data-insert-line="" />
                    }.into_any())
                } else { None })
                .collect_view()
                .into_any()
            }}
        </div>
    }
}
