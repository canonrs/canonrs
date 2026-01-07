use leptos::prelude::*;

#[component]
pub fn DataGridFooter(
    pagination: bool,
    current_page: RwSignal<usize>,
    total_pages: Signal<usize>,
    total_rows: Signal<usize>,
) -> impl IntoView {
    view! {
        <div class="border-t">
            {move || {
                if pagination && total_pages.get() > 1 {
                    view! {
                        <div class="flex items-center justify-between px-4 py-3">
                            <div class="text-sm text-muted-foreground">
                                "Page " {move || current_page.get() + 1} " of " {move || total_pages.get()}
                                " (" {move || total_rows.get()} " rows)"
                            </div>
                            <div class="flex gap-2">
                                <button
                                    class="px-3 py-1 text-sm border rounded hover:bg-muted disabled:opacity-50"
                                    disabled=move || current_page.get() == 0
                                    on:click=move |_| current_page.update(|p| { if *p > 0 { *p -= 1; } })
                                >
                                    "Previous"
                                </button>
                                <button
                                    class="px-3 py-1 text-sm border rounded hover:bg-muted disabled:opacity-50"
                                    disabled=move || current_page.get() >= total_pages.get() - 1
                                    on:click=move |_| current_page.update(|p| { if *p < total_pages.get() - 1 { *p += 1; } })
                                >
                                    "Next"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="px-4 py-2 text-sm text-muted-foreground">
                            {move || total_rows.get()} " rows total"
                            {if !pagination { " (virtual scroll)" } else { "" }}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
