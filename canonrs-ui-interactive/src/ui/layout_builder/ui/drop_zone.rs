use leptos::prelude::*;
use crate::ui::layout_builder::types::{Node, DragContext, CanvasMode, children_of};
use super::block_preview::BlockPreview;
pub use super::super::state::drop_zone_types::DragVisualState;
use super::drop_zone_virt::{VirtState, VIRT_THRESHOLD, compute_offsets, compute_visible_range};
use super::drop_handler::handle_drop;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;

#[component]
pub fn DropZone(
    parent_id: uuid::Uuid,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
    #[prop(optional)] slot_label: Option<String>,
    #[prop(optional)] virtualize: bool,
) -> impl IntoView {
    let is_builder     = move || canvas_mode.get() == CanvasMode::Builder;
    let is_dragging    = move || drag_ctx.get().is_dragging();
    let is_layout_drag = move || drag_ctx.get().layout_def.is_some();
    let is_active      = move || drag_visual.get().active_zone_id == Some(parent_id);
    let insert_idx     = move || drag_visual.get().insert_index;
    let _label = slot_label.unwrap_or_else(|| "drop".to_string());

    let children_memo = Memo::new(move |_| children_of(&tree.get(), parent_id));
    let virt = VirtState::new();
    let scroll_el: NodeRef<leptos::html::Div> = NodeRef::new();
    let zone_el:   NodeRef<leptos::html::Div> = NodeRef::new();

    let should_virt = move || virtualize && !is_dragging() && children_memo.get().len() > VIRT_THRESHOLD;

    let offsets = Memo::new(move |_| {
        compute_offsets(&virt.item_heights.get(), children_memo.get().len())
    });

    let visible_range = Memo::new(move |_| {
        if !should_virt() { return (0, children_memo.get().len()); }
        let acc = offsets.get();
        let n = acc.len().saturating_sub(1);
        compute_visible_range(virt.scroll_top.get(), virt.container_height.get(), &acc, n)
    });

    let virt_scroll_top = virt.scroll_top;
    let virt_container_height = virt.container_height;
    let virt_item_heights = virt.item_heights;

    let handle_scroll = move |_: leptos::ev::Event| {
        #[cfg(target_arch = "wasm32")] {
            use leptos::wasm_bindgen::JsCast;
            if let Some(el) = scroll_el.get() {
                let el: web_sys::Element = el.unchecked_into();
                virt_scroll_top.set(el.scroll_top() as f64);
                virt_container_height.set(el.client_height() as f64);
            }
        }
    };

    Effect::new(move |_| {
        let _ = children_memo.get();
        #[cfg(target_arch = "wasm32")] {
            use leptos::wasm_bindgen::JsCast;
            if let Some(el) = zone_el.get() {
                let el: web_sys::Element = el.unchecked_into();
                if let Ok(blocks) = el.query_selector_all(":scope > [data-block-preview]") {
                    let heights: Vec<f64> = (0..blocks.length())
                        .filter_map(|i| blocks.item(i))
                        .map(|b| { let be: web_sys::Element = b.unchecked_into(); be.get_bounding_client_rect().height() })
                        .collect();
                    virt_item_heights.set(heights);
                }
            }
            if let Some(el) = scroll_el.get() {
                let el: web_sys::Element = el.unchecked_into();
                let h = el.client_height() as f64;
                if h > 0.0 { virt_container_height.set(h); }
            }
        }
    });

    let insert_line = move |pos: usize| view! {
        <div
            data-insert-line=""
            attr:data-active=move || if is_active() && is_builder() && insert_idx() == pos { "true" } else { "false" }
        />
    };

    view! {
        {move || if should_virt() {
            view! {
                <div
                    node_ref=scroll_el
                    data-drop-zone-scroll=""
                    on:scroll=handle_scroll
                >
                    <div
                        node_ref=zone_el
                        data-drop-zone=""
                        data-zone-id=parent_id.to_string()
                        attr:data-dragging=move || if is_dragging() && !is_layout_drag() { "true" } else { "false" }
                        attr:data-mode=move || if is_builder() { "builder" } else { "preview" }
                    >
                        {move || {
                            let top = offsets.get().first().copied().unwrap_or(0.0);
                            view! { <div style=move || format!("height:{top}px;pointer-events:none;") /> }
                        }}
                        {insert_line(0)}
                        <For
                            each=move || {
                                let ch = children_memo.get();
                                let (s, e) = visible_range.get();
                                ch.into_iter().skip(s).take(e.saturating_sub(s)).collect::<Vec<_>>()
                            }
                            key=|n| n.id
                            children=move |n| {
                                let pos = children_memo.get().iter().position(|c| c.id == n.id).unwrap_or(0);
                                view! {
                                    <BlockPreview node=n engine=engine tree=tree drag_ctx=drag_ctx
                                        selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual />
                                    {insert_line(pos + 1)}
                                }
                            }
                        />
                        {move || {
                            let acc = offsets.get();
                            let n = acc.len().saturating_sub(1);
                            let (_, end) = visible_range.get();
                            let bot = acc.get(n).copied().unwrap_or(0.0) - acc.get(end).copied().unwrap_or(0.0);
                            view! { <div style=move || format!("height:{bot}px;pointer-events:none;") /> }
                        }}
                    </div>
                </div>
            }.into_any()
        } else {
            view! {
                <div
                    node_ref=zone_el
                    data-drop-zone=""
                    data-zone-id=parent_id.to_string()
                    attr:data-dragging=move || if is_dragging() && !is_layout_drag() { "true" } else { "false" }
                    attr:data-mode=move || if is_builder() { "builder" } else { "preview" }
                >
                    {insert_line(0)}
                    <For
                        each=move || children_memo.get()
                        key=|n| n.id
                        children=move |n| {
                            let pos = children_memo.get().iter().position(|c| c.id == n.id).unwrap_or(0);
                            view! {
                                <BlockPreview node=n engine=engine tree=tree drag_ctx=drag_ctx
                                    selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual />
                                {insert_line(pos + 1)}
                            }
                        }
                    />
                </div>
            }.into_any()
        }}
    }
}
