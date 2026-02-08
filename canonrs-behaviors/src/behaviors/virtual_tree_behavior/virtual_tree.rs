use leptos::prelude::*;
use wasm_bindgen::JsCast;
use crate::primitives::tree::Tree as TreePrimitive;
use crate::primitives::virtual_list::{VirtualListViewport, VirtualListContent};
use super::types::ScrollState;
use super::flattener::flatten_tree;
use super::viewport::{calculate_visible_range, calculate_total_height, calculate_offset};
use super::virtual_row::VirtualTreeRow;

#[component]
pub fn VirtualTree(
    nodes: Signal<Vec<crate::ui::tree::TreeNode>>,
    selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
    #[prop(default = 36.0)] row_height: f64,
    #[prop(default = 600.0)] viewport_height: f64,
    #[prop(default = 5)] overscan: usize,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let scroll_state = RwSignal::new(ScrollState { scroll_top: 0.0 });

    let flat_nodes = Memo::new(move |_| {
        flatten_tree(&nodes.get())
    });

    let total_visible = Memo::new(move |_| {
        flat_nodes.get().len()
    });

    let total_height = Memo::new(move |_| {
        calculate_total_height(total_visible.get(), row_height)
    });

    let visible_range = Memo::new(move |_| {
        calculate_visible_range(
            scroll_state.get(),
            total_visible.get(),
            row_height,
            viewport_height,
            overscan,
        )
    });

    let visible_nodes = Memo::new(move |_| {
        let range = visible_range.get();
        let nodes = flat_nodes.get();
        nodes.into_iter()
            .skip(range.start)
            .take(range.end - range.start)
            .collect::<Vec<_>>()
    });

    let offset_top = Memo::new(move |_| {
        calculate_offset(visible_range.get(), row_height)
    });

    let handle_scroll = move |ev: web_sys::Event| {
        if let Some(target) = ev.target() {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                scroll_state.set(ScrollState {
                    scroll_top: element.scroll_top() as f64,
                });
            }
        }
    };

    view! {
        <TreePrimitive
            class={class}
            id={id}
        >
            <VirtualListViewport
                on:scroll=handle_scroll
            >
                <VirtualListContent>
                    <div style={move || format!("height: {}px", total_height.get())}>
                        <div style={move || format!("transform: translateY({}px)", offset_top.get())}>
                            {move || {
                                visible_nodes.get().into_iter().map(|node| {
                                    view! {
                                        <VirtualTreeRow
                                            row_height=row_height
                                            node=node.clone()
                                            selected={selected_id.get() == Some(node.id.clone())}
                                            on_select=on_select
                                            on_toggle=on_toggle
                                        />
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </div>
                </VirtualListContent>
            </VirtualListViewport>
        </TreePrimitive>
    }
}
