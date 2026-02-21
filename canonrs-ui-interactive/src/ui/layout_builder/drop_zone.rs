use leptos::prelude::*;
use super::types::{Node, DragContext, CanvasMode, children_of, insert_node, remove_node, move_node};
use super::block_preview::BlockPreview;

#[component]
pub fn DropZone(
    parent_id: uuid::Uuid,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    #[prop(optional)] slot_label: Option<&'static str>,
) -> impl IntoView {
    let is_builder = move || canvas_mode.get() == CanvasMode::Builder;
    let hover = RwSignal::new(false);
    let insert_index: RwSignal<usize> = RwSignal::new(0);
    let is_dragging = move || drag_ctx.get().is_dragging();
    let get_children = move || children_of(&tree.get(), parent_id);

    // ── Virtual Drop Targets Cache ─────────────────────────────────────────────
    // Armazena midpoints dos blocos filhos — calculado UMA vez por drag session
    // pointermove só faz lookup em memória, zero DOM query
    let rect_cache: RwSignal<Vec<f64>> = RwSignal::new(vec![]);
    let cache_valid = RwSignal::new(false);
    let zone_el: NodeRef<leptos::html::Div> = NodeRef::new();

    // Invalida cache quando tree ou drag muda
    Effect::new(move |_| {
        let _ = tree.get();
        let _ = drag_ctx.get();
        cache_valid.set(false);
    });

    let build_cache = move || {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;
            if let Some(el) = zone_el.get() {
                let el: web_sys::Element = el.unchecked_into();
                if let Ok(children) = el.query_selector_all(":scope > [data-block-preview]") {
                    let len = children.length() as usize;
                    let mut mids = Vec::with_capacity(len);
                    for i in 0..len {
                        if let Some(child) = children.item(i as u32) {
                            let child_el: web_sys::Element = child.unchecked_into();
                            let rect = child_el.get_bounding_client_rect();
                            mids.push(rect.top() + rect.height() / 2.0);
                        }
                    }
                    rect_cache.set(mids);
                    cache_valid.set(true);
                }
            }
        }
    };

    let handle_pointermove = move |ev: leptos::ev::PointerEvent| {
        if !is_dragging() { return; }
        hover.set(true);

        // Constrói cache se inválido
        if !cache_valid.get() {
            build_cache();
        }

        // Lookup em memória — zero DOM query
        let client_y = ev.client_y() as f64;
        let mids = rect_cache.get();
        let len = mids.len();
        let mut idx = len;
        for (i, &mid) in mids.iter().enumerate() {
            if client_y < mid {
                idx = i;
                break;
            }
        }
        insert_index.set(idx);
    };

    let handle_pointerup = move |ev: leptos::ev::PointerEvent| {
        ev.stop_propagation();
        let ctx = drag_ctx.get();
        if !ctx.is_dragging() || !hover.get() { return; }
        let block = match ctx.block_def { Some(b) => b, None => return };

        let parent_node = tree.get().iter().find(|n| n.id == parent_id).cloned();
        if let Some(ref p) = parent_node {
            if !p.accepts(&block) {
                hover.set(false);
                return;
            }
        }

        let idx = insert_index.get();
        if let Some(src_id) = ctx.node_id {
            tree.update(|t| move_node(t, src_id, parent_id, idx));
        } else {
            let node = Node::block(block, parent_id, idx);
            tree.update(|t| insert_node(t, node));
        }

        cache_valid.set(false);
        hover.set(false);
        drag_ctx.set(DragContext::empty());
    };

    let handle_pointerleave = move |_: leptos::ev::PointerEvent| {
        hover.set(false);
    };

    let label = slot_label.unwrap_or("drop");

    view! {
        <div
            node_ref=zone_el
            data-drop-zone=""
            attr:data-hover=move || if hover.get() { "true" } else { "false" }
            attr:data-dragging=move || if is_dragging() { "true" } else { "false" }
            attr:data-mode=move || if is_builder() { "builder" } else { "preview" }
            on:pointermove=move |ev| { if is_builder() { handle_pointermove(ev); } }
            on:pointerup=move |ev| { if is_builder() { handle_pointerup(ev); } }
            on:pointerleave=move |ev| { if is_builder() { handle_pointerleave(ev); } }
        >
            {move || {
                let nodes = get_children();
                let idx = insert_index.get();
                let hovering = hover.get() && is_builder();

                if nodes.is_empty() {
                    return if is_builder() {
                        view! { <div data-drop-zone-empty="">{format!("Drop here → {}", label)}</div> }.into_any()
                    } else {
                        view! { <div /> }.into_any()
                    };
                }

                let mut views: Vec<AnyView> = Vec::new();

                // Drop line antes do primeiro bloco
                views.push(view! {
                    <div style=move || if hovering && idx == 0 {
                        "height: 4px; background: var(--builder-insert-line-color); border-radius: 2px; margin: 2px 0; pointer-events: none;"
                    } else {
                        "height: 2px; background: transparent; margin: 1px 0; pointer-events: none;"
                    } />
                }.into_any());

                for (i, n) in nodes.into_iter().enumerate() {
                    let after = i + 1;
                    views.push(view! {
                        <BlockPreview
                            node=n
                            tree=tree
                            drag_ctx=drag_ctx
                            selected_id=selected_id
                            canvas_mode=canvas_mode
                        />
                    }.into_any());
                    views.push(view! {
                        <div style=move || if hovering && idx == after {
                            "height: 4px; background: var(--builder-insert-line-color); border-radius: 2px; margin: 2px 0; pointer-events: none;"
                        } else {
                            "height: 2px; background: transparent; margin: 1px 0; pointer-events: none;"
                        } />
                    }.into_any());
                }

                views.into_iter().collect_view().into_any()
            }}
        </div>
    }
}
