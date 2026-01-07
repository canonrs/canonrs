use leptos::prelude::*;
use wasm_bindgen::JsCast;
use super::types::{VirtualTreeNode, ViewportRange, ScrollState};
use super::virtual_row::VirtualTreeRow;

#[component]
pub fn VirtualTreeViewport(
    #[prop(into)] nodes: Signal<Vec<VirtualTreeNode>>,
    #[prop(into)] selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
    #[prop(default = 36.0)] row_height: f64,
    #[prop(default = 600.0)] viewport_height: f64,
    #[prop(default = 5)] overscan: usize,
) -> impl IntoView {
    let scroll_state = RwSignal::new(ScrollState { scroll_top: 0.0 });

    let visible_range = Signal::derive(move || {
        let scroll_top = scroll_state.get().scroll_top;
        let total_rows = nodes.get().len();

        let start = ((scroll_top / row_height).floor() as usize).saturating_sub(overscan);
        let visible_count = (viewport_height / row_height).ceil() as usize;
        let end = (start + visible_count + overscan * 2).min(total_rows);

        ViewportRange { start, end }
    });

    let total_height = Signal::derive(move || {
        nodes.get().len() as f64 * row_height
    });

    let on_scroll = move |ev: web_sys::Event| {
        if let Some(target) = ev.target() {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                scroll_state.update(|s| {
                    s.scroll_top = element.scroll_top() as f64;
                });
            }
        }
    };

    view! {
        <div
            class="virtual-tree-viewport overflow-y-auto"
            style=format!("height: {}px", viewport_height)
            on:scroll=on_scroll
        >
            <div style=move || format!("height: {}px; position: relative", total_height.get())>
                <div style=move || {
                    let range = visible_range.get();
                    let offset = range.start as f64 * row_height;
                    format!("position: absolute; top: {}px; left: 0; right: 0", offset)
                }>
                    {move || {
                        let range = visible_range.get();
                        let all_nodes = nodes.get();
                        let selected = selected_id.get();

                        all_nodes
                            .into_iter()
                            .skip(range.start)
                            .take(range.end - range.start)
                            .map(|node| {
                                let is_selected = selected.as_ref() == Some(&node.id);
                                view! {
                                    <VirtualTreeRow
                                        node=node
                                        row_height=row_height
                                        selected=is_selected
                                        on_select=on_select
                                        on_toggle=on_toggle
                                    />
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}
