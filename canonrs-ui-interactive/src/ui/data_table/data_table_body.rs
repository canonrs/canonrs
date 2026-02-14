use leptos::prelude::*;
use canonrs_ui::primitives::{
    DataTableBodyPrimitive,
    DataTableRowPrimitive,
    DataTableCellPrimitive,
};
use super::types::ColumnDef;
use super::PinPosition;

#[component]
pub fn DataTableBody<T: Clone + PartialEq + Send + Sync + 'static>(
    columns: Signal<Vec<ColumnDef<T>>>,
    data: RwSignal<Vec<T>>,
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
    #[prop(optional)] column_widths: Option<RwSignal<std::collections::HashMap<String, u32>>>,
    #[prop(optional)] pinned_columns: Option<RwSignal<std::collections::HashMap<String, PinPosition>>>,
    #[prop(optional)] pin_offsets: Option<Signal<std::collections::HashMap<String, u32>>>,
) -> impl IntoView {

    view! {
        <DataTableBodyPrimitive>
            {move || {
                if loading.get() {
                    return view! {
                        <DataTableRowPrimitive id="loading">
                            <DataTableCellPrimitive class="text-center py-8">
                                "Loading..."
                            </DataTableCellPrimitive>
                        </DataTableRowPrimitive>
                    }.into_any();
                }

                if let Some(err) = error.get() {
                    return view! {
                        <DataTableRowPrimitive id="error">
                            <DataTableCellPrimitive class="text-center py-8 text-red-600">
                                {err}
                            </DataTableCellPrimitive>
                        </DataTableRowPrimitive>
                    }.into_any();
                }

                let rows = data.get();
                let cols = columns.get();

                if rows.is_empty() {
                    return view! {
                        <DataTableRowPrimitive id="empty">
                            <DataTableCellPrimitive class="text-center py-8">
                                "No data found"
                            </DataTableCellPrimitive>
                        </DataTableRowPrimitive>
                    }.into_any();
                }

                rows.into_iter().enumerate().map(|(idx, item)| {
                    let cols_for_row = cols.clone();
                    view! {
                        <DataTableRowPrimitive id=idx.to_string()>
                            {cols_for_row.into_iter().map(|col| {
                                let rendered = (col.render)(&item);
                                let col_id   = col.id.clone();

                                view! {
                                    <DataTableCellPrimitive
                                        style=Signal::derive(move || {
                                            if let Some(pins) = pinned_columns {
                                                pins.with(|map| match map.get(&col_id) {
                                                    Some(PinPosition::Left) => {
                                                        let offset = pin_offsets.map(|o| o.with(|m| m.get(&format!("left:{}", col_id)).copied().unwrap_or(0))).unwrap_or(0);
                                                        format!("position: sticky; left: {}px; z-index: 1; background: var(--data-table-bg, var(--color-background))", offset)
                                                    },
                                                    Some(PinPosition::Right) => {
                                                        let offset = pin_offsets.map(|o| o.with(|m| m.get(&format!("right:{}", col_id)).copied().unwrap_or(0))).unwrap_or(0);
                                                        format!("position: sticky; right: {}px; z-index: 1; background: var(--data-table-bg, var(--color-background))", offset)
                                                    },
                                                    _ => String::new(),
                                                })
                                            } else {
                                                String::new()
                                            }
                                        })
                                    >
                                        {rendered}
                                    </DataTableCellPrimitive>
                                }
                            }).collect_view()}
                        </DataTableRowPrimitive>
                    }
                }).collect_view().into_any()
            }}
        </DataTableBodyPrimitive>
    }
}
