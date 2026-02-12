use leptos::prelude::*;
use canonrs_ui::primitives::{
    DataTableBodyPrimitive, DataTableRowPrimitive, DataTableCellPrimitive,
};
use super::types::ColumnDef;

#[component]
pub fn DataTableBody<T: Clone + PartialEq + Send + Sync + 'static>(
    columns: Signal<Vec<ColumnDef<T>>>,
    data: RwSignal<Vec<T>>,
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <DataTableBodyPrimitive>
            {move || {
                let rows_owned = data.with(|rows| rows.clone());
                let cols_owned = columns.with(|cols| cols.clone());

                if loading.get() {
                    view! {
                        <DataTableRowPrimitive id="loading">
                            <DataTableCellPrimitive class="text-center py-8">
                                "Loading..."
                            </DataTableCellPrimitive>
                        </DataTableRowPrimitive>
                    }.into_any()
                } else if let Some(err) = error.get() {
                    view! {
                        <DataTableRowPrimitive id="error">
                            <DataTableCellPrimitive class="text-center py-8 text-red-600">
                                {err}
                            </DataTableCellPrimitive>
                        </DataTableRowPrimitive>
                    }.into_any()
                } else if rows_owned.is_empty() {
                    view! {
                        <DataTableRowPrimitive id="empty">
                            <DataTableCellPrimitive class="text-center py-8">
                                "No data found"
                            </DataTableCellPrimitive>
                        </DataTableRowPrimitive>
                    }.into_any()
                } else {
                    rows_owned.into_iter().enumerate().map(|(idx, item)| {
                        let cols = cols_owned.clone();
                        view! {
                            <DataTableRowPrimitive id=idx.to_string()>
                                {cols.iter().map(|col| {
                                    let rendered = (col.render)(&item);
                                    view! {
                                        <DataTableCellPrimitive>{rendered}</DataTableCellPrimitive>
                                    }
                                }).collect_view()}
                            </DataTableRowPrimitive>
                        }
                    }).collect_view().into_any()
                }
            }}
        </DataTableBodyPrimitive>
    }
}
