//! Virtual Viewport Component
//! Manages scroll state and visible range calculation

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use super::types::{VirtualRow, VirtualColumn};
use super::math::*;
use super::virtual_row::VirtualRowComponent;

#[component]
pub fn VirtualViewport(
    rows: Signal<Vec<VirtualRow>>,
    columns: Vec<VirtualColumn>,
    row_height: f64,
    viewport_height: f64,
    #[prop(default = 5)] overscan: usize,
) -> impl IntoView {
    let scroll_top = RwSignal::new(0.0);

    let visible_range = Memo::new(move |_| {
        calculate_visible_range(
            scroll_top.get(),
            viewport_height,
            row_height,
            rows.get().len(),
            overscan,
        )
    });

    let total_height = Memo::new(move |_| {
        calculate_total_height(rows.get().len(), row_height)
    });

    let offset = Memo::new(move |_| {
        let range = visible_range.get();
        calculate_offset(range.start, row_height)
    });

    let visible_rows = Memo::new(move |_| {
        let range = visible_range.get();
        let all_rows = rows.get();

        all_rows
            .iter()
            .skip(range.start)
            .take(range.end - range.start)
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <div
            attr:data-virtual-viewport=""
            style={format!("--viewport-height: {}px;", viewport_height)}
            on:scroll={move |evt| {
                if let Some(element) = evt.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) {
                    scroll_top.set(element.scroll_top() as f64);
                }
            }}
        >
            <div
                attr:data-virtual-spacer=""
                style={move || format!("--total-height: {}px;", total_height.get())}
            >
                <div
                    attr:data-virtual-content=""
                    style={move || format!("--offset: {}px;", offset.get())}
                >
                    {move || visible_rows.get().into_iter().map(|row| {
                        view! {
                            <VirtualRowComponent
                                row={row}
                                columns={columns.clone()}
                                row_height={row_height}
                            />
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}
