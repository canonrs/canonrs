use leptos::prelude::*;
use leptos::logging::log;
use wasm_bindgen::JsCast;
use super::types::{VirtualRow, VirtualColumn};
use super::math::*;
use super::virtual_row::VirtualRowComponent;

#[component]
pub fn VirtualViewport(
    rows: Signal<Vec<VirtualRow>>,
    columns: Vec<VirtualColumn>,
    #[prop(into)] row_height: f64,
    #[prop(into)] viewport_height: f64,
    #[prop(default = 5)] overscan: usize,
) -> impl IntoView {
    let scroll_top = RwSignal::new(0.0);
    
    let visible_range = Memo::new(move |_| {
        let range = calculate_visible_range(
            scroll_top.get(),
            viewport_height,
            row_height,
            rows.get().len(),
            overscan,
        );
        range
    });
    
    let total_height = Memo::new(move |_| {
        calculate_total_height(rows.get().len(), row_height)
    });
    
    // ✅ CORRETO - Derivar do visible_range explicitamente
    let offset = Memo::new(move |_| {
        let range = visible_range.get(); // força dependência
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
    
    let handle_scroll = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let element: web_sys::HtmlElement = target.dyn_into().unwrap();
        scroll_top.set(element.scroll_top() as f64);
    };
    
    view! {
        <div 
            class="virtual-viewport overflow-y-auto bg-surface"
            style=format!("height: {}px;", viewport_height)
            on:scroll=handle_scroll
        >
            <div 
                class="virtual-spacer"
                style=format!("height: {}px; position: relative;", total_height.get())
            >
                <div 
                    class="virtual-content"
                    style=move || format!("transform: translateY({}px);", offset.get())
                >
                    {move || visible_rows.get().into_iter().map(|row| {
                        view! {
                            <VirtualRowComponent 
                                row=row
                                columns=columns.clone()
                                row_height=row_height
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
