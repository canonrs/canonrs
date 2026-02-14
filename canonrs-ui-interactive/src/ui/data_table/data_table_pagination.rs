use leptos::prelude::*;
use canonrs_ui::primitives::DataTablePaginationPrimitive;

#[component]
pub fn DataTablePagination(
    current_page: RwSignal<usize>,
    total: RwSignal<usize>,
    page_size: usize,
) -> impl IntoView {
    let handle_prev = move |_| {
        current_page.update(|page| {
            if *page > 1 { *page -= 1; }
        });
    };

    let handle_next = move |_| {
        let t = total.get();
        let total_pages = ((t as f32) / (page_size as f32)).ceil() as usize;
        current_page.update(|page| {
            if *page < total_pages { *page += 1; }
        });
    };

    view! {
        <DataTablePaginationPrimitive>
            <div>
                {move || {
                    let t = total.get();
                    let page = current_page.get();
                    let start = (page - 1) * page_size + 1;
                    let end = (start + page_size).saturating_sub(1).min(t);
                    format!("Showing {}-{} of {}", start, end, t)
                }}
            </div>
            <div class="flex gap-2">
                <button
                    on:click=handle_prev
                    class="px-3 py-1 border rounded"
                    disabled=move || current_page.get() == 1
                >"Previous"</button>
                <button
                    on:click=handle_next
                    class="px-3 py-1 border rounded"
                    disabled=move || {
                        let t = total.get();
                        let total_pages = ((t as f32) / (page_size as f32)).ceil() as usize;
                        current_page.get() >= total_pages
                    }
                >"Next"</button>
            </div>
        </DataTablePaginationPrimitive>
    }
}
