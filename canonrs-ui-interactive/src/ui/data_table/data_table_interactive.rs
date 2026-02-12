use leptos::prelude::*;
use canonrs_ui::primitives::{
    DataTableToolbarPrimitive, DataTableScrollPrimitive,
    DataTableTablePrimitive,
};
use super::types::{DataTableRequest, DataTableResponse, ColumnDef};
use super::state::DataTableState;
use super::data_table_header::DataTableHeader;
use super::data_table_body::DataTableBody;
use super::data_table_pagination::DataTablePagination;

#[component]
pub fn DataTableInteractive<T, F>(
    columns: Signal<Vec<ColumnDef<T>>>,
    fetch_data: F,
    #[prop(default = 20)] page_size: usize,
    #[prop(into, optional)] id: Option<String>,
    #[prop(optional)] density: Option<Signal<String>>,
    #[prop(optional)] zebra: Option<Signal<bool>>,
    #[prop(optional)] row_hover: Option<Signal<bool>>,
    #[prop(optional)] sticky_header: Option<Signal<bool>>,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(DataTableRequest) -> Result<DataTableResponse<T>, String> + 'static,
{
    let state: DataTableState<T> = DataTableState::new(columns, fetch_data, page_size);

    let data_len = RwSignal::new(0usize);
    create_effect(move |_| {
        data_len.set(state.data.with(|rows: &Vec<T>| rows.len()));
    });

    let density_signal  = density.unwrap_or_else(|| Signal::derive(|| "comfortable".to_string()));
    let zebra_signal    = zebra.unwrap_or_else(|| Signal::derive(|| false));
    let hover_signal    = row_hover.unwrap_or_else(|| Signal::derive(|| true));
    let sticky_signal   = sticky_header.unwrap_or_else(|| Signal::derive(|| false));
    let table_id        = id.unwrap_or_else(|| "datatable-interactive".to_string());

    view! {
        <div
            data-datatable=""
            data-density=move || density_signal.get()
            data-zebra=move || zebra_signal.get().to_string()
            data-row-hover=move || hover_signal.get().to_string()
            data-sticky-header=move || sticky_signal.get().to_string()
            id=table_id
        >
            <DataTableToolbarPrimitive>
                <input
                    type="text"
                    placeholder="Search..."
                    class="px-3 py-2 border rounded w-64"
                    on:input=move |ev| {
                        state.filter_query.set(event_target_value(&ev));
                        state.current_page.set(1);
                    }
                />
            </DataTableToolbarPrimitive>

            <DataTableScrollPrimitive>
                <DataTableTablePrimitive>
                    <DataTableHeader<T>
                        columns=columns
                        sort_column=state.sort_column
                        sort_ascending=state.sort_ascending
                        current_page=state.current_page
                    />
                    <DataTableBody<T>
                        columns=columns
                        data=state.data
                        loading=state.loading
                        error=state.error
                    />
                </DataTableTablePrimitive>
            </DataTableScrollPrimitive>

            <DataTablePagination
                current_page=state.current_page
                total=state.total
                data_len=data_len
                page_size=page_size
            />
        </div>
    }
}
