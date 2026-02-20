use leptos::prelude::*;
use super::types::{DroppedBlock, LayoutRegion, DragContext, all_blocks};
use super::block_preview::BlockPreview;

#[component]
pub fn DropZone(
    region: LayoutRegion,
    dropped: RwSignal<Vec<DroppedBlock>>,
    drag_ctx: RwSignal<DragContext>,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let hover = RwSignal::new(false);
    let insert_index: RwSignal<Option<usize>> = RwSignal::new(None);
    let is_dragging = move || drag_ctx.get().is_dragging();

    let region_blocks = move || {
        let mut blocks: Vec<_> = dropped.get().into_iter()
            .filter(|d| d.region == region)
            .collect();
        blocks.sort_by_key(|d| d.order);
        blocks
    };

    let handle_pointermove = move |ev: leptos::ev::PointerEvent| {
        if !is_dragging() { return; }
        hover.set(true);
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsValue;
            use leptos::wasm_bindgen::JsCast;
            // Encontrar blocos filhos e calcular índice pelo midpoint real
            let client_y = ev.client_y() as f64;
            let target = ev.current_target();
            if let Some(el) = target {
                let el: web_sys::Element = el.unchecked_into();
                let children = el.query_selector_all("[data-block-preview]").ok();
                if let Some(nodes) = children {
                    let len = nodes.length() as usize;
                    let mut idx = len;
                    for i in 0..len {
                        if let Some(node) = nodes.item(i as u32) {
                            let child_el: web_sys::Element = node.unchecked_into();
                            let rect = child_el.get_bounding_client_rect();
                            let mid = rect.top() + rect.height() / 2.0;
                            if client_y < mid {
                                idx = i;
                                break;
                            }
                        }
                    }
                    insert_index.set(Some(idx));
                }
            }
        }
    };

    let handle_pointerup = move |_: leptos::ev::PointerEvent| {
        let ctx = drag_ctx.get();
        if !ctx.is_dragging() { return; }
        if !hover.get() { return; }

        let block = match ctx.block_def { Some(b) => b, None => return };

        // Remove da origem se for reposicionamento
        if let Some(src_id) = ctx.instance_id {
            dropped.update(|v| {
                if let Some(pos) = v.iter().position(|d| d.instance_id == src_id) {
                    v.remove(pos);
                }
            });
        }

        let idx = insert_index.get().unwrap_or_else(|| region_blocks().len());

        dropped.update(|v| {
            // Reordena blocos existentes da região
            let mut region_items: Vec<usize> = v.iter().enumerate()
                .filter(|(_, d)| d.region == region)
                .map(|(i, _)| i)
                .collect();
            region_items.sort_by_key(|&i| v[i].order);

            for (new_order, &vec_idx) in region_items.iter().enumerate() {
                let adjusted = if new_order >= idx { new_order + 1 } else { new_order };
                v[vec_idx].order = adjusted;
            }

            v.push(DroppedBlock {
                instance_id: uuid::Uuid::new_v4(),
                block,
                region,
                order: idx,
            });
        });

        insert_index.set(None);
        hover.set(false);
        drag_ctx.set(DragContext::empty());
    };

    let handle_pointerleave = move |_: leptos::ev::PointerEvent| {
        hover.set(false);
        insert_index.set(None);
    };

    view! {
        <div
            class=class
            style=move || format!(
                "min-height: 60px; border: 2px dashed {}; border-radius: var(--radius-sm); padding: 0.5rem; transition: border-color 0.15s, background 0.15s; background: {}; position: relative; touch-action: none;",
                if hover.get() { "#6366f1" } else if is_dragging() { "#a5b4fc" } else { "var(--theme-surface-border)" },
                if hover.get() { "rgba(99,102,241,0.12)" } else if is_dragging() { "rgba(99,102,241,0.06)" } else { "transparent" }
            )
            data-drop-region=region.as_str()
            on:pointermove=handle_pointermove
            on:pointerup=handle_pointerup
            on:pointerleave=handle_pointerleave
        >
            {children.map(|c| c())}

            {move || {
                let blocks = region_blocks();
                let idx = insert_index.get();
                let hovering = hover.get();
                blocks.into_iter().enumerate().flat_map(|(i, d)| {
                    let line = if hovering && idx == Some(i) {
                        Some(view! {
                            <div style="height: 3px; background: #6366f1; border-radius: 2px; margin: 2px 0; transition: all 0.1s;" />
                        }.into_any())
                    } else { None };
                    vec![
                        line,
                        Some(view! {
                            <div data-block-preview="">
                                <BlockPreview instance_id=d.instance_id block=d.block region=region dropped=dropped drag_ctx=drag_ctx />
                            </div>
                        }.into_any()),
                    ].into_iter().flatten()
                }).chain({
                    let show_end = hover.get() && insert_index.get() >= Some(region_blocks().len());
                    if show_end {
                        Some(view! {
                            <div style="height: 3px; background: #6366f1; border-radius: 2px; margin: 2px 0;" />
                        }.into_any())
                    } else { None }
                }).collect_view()
            }}

            <Show when=move || region_blocks().is_empty()>
                <div style="font-size: 0.7rem; color: var(--theme-surface-fg-muted); text-align: center; padding: 0.5rem; pointer-events: none;">
                    {format!("Drop here → {}", region.as_str())}
                </div>
            </Show>
        </div>
    }
}
