use leptos::prelude::*;
use canonrs_ui::primitives::{
    DataTableHeadPrimitive, DataTableHeadRowPrimitive, DataTableHeadCellPrimitive,
};
use super::types::ColumnDef;

#[component]
pub fn DataTableHeader<T: Clone + PartialEq + Send + Sync + 'static>(
    columns: Signal<Vec<ColumnDef<T>>>,
    sort_column: RwSignal<Option<String>>,
    sort_ascending: RwSignal<bool>,
    current_page: RwSignal<usize>,
) -> impl IntoView {
    let handle_sort = move |col: String| {
        if sort_column.get().as_ref() == Some(&col) {
            sort_ascending.update(|asc| *asc = !*asc);
        } else {
            sort_column.set(Some(col));
            sort_ascending.set(true);
        }
        current_page.set(1);
    };

    view! {
        <DataTableHeadPrimitive>
            <DataTableHeadRowPrimitive>
                {move || {
                    columns.with(|cols| {
                        cols.iter().map(|col| {
                            let col_id = col.id.clone();
                            let col_label = col.label.clone();
                            let col_sortable = col.sortable;
                            let col_width = col.width.clone();
                            let col_id_for_sort = col.id.clone();
                            let col_id_for_indicator = col.id.clone();

                            view! {
                                <DataTableHeadCellPrimitive
                                    sort_key=if col_sortable { col_id.clone() } else { String::new() }
                                    class=col_width.unwrap_or_default()
                                >
                                    {if col_sortable {
                                        view! {
                                            <button
                                                on:click=move |_| handle_sort(col_id_for_sort.clone())
                                                class="w-full text-left flex items-center gap-2"
                                            >
                                                {col_label.clone()}
                                                <span class="sort-indicator">
                                                    {move || {
                                                        if sort_column.get().as_ref() == Some(&col_id_for_indicator) {
                                                            if sort_ascending.get() { "▲" } else { "▼" }
                                                        } else { "⇅" }
                                                    }}
                                                </span>
                                            </button>
                                        }.into_any()
                                    } else {
                                        view! { {col_label} }.into_any()
                                    }}
                                </DataTableHeadCellPrimitive>
                            }
                        }).collect_view()
                    })
                }}
            </DataTableHeadRowPrimitive>
        </DataTableHeadPrimitive>
    }
}
