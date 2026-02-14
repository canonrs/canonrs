use leptos::prelude::*;
use canonrs_ui::primitives::{
    DataTableToolbarPrimitive,
};
use super::types::{DataTableRequest, DataTableResponse, ColumnDef};
use super::PinPosition;
use super::state::DataTableState;
use super::data_table_header::DataTableHeader;
use super::data_table_body::DataTableBody;
use super::data_table_pagination::DataTablePagination;

fn col_width_px(col: &ColumnDef<impl Clone>, widths: &std::collections::HashMap<String, u32>) -> String {
    widths.get(&col.id)
        .map(|w| format!("{}px", w))
        .or_else(|| col.width.clone())
        .unwrap_or_else(|| "150px".to_string())
}

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
    #[prop(optional)] draggable: Option<Signal<bool>>,
    #[prop(optional)] resizable: Option<Signal<bool>>,
    #[prop(optional)] pinnable: Option<Signal<bool>>,
    #[prop(optional)] column_widths: Option<RwSignal<std::collections::HashMap<String, u32>>>,
    #[prop(optional)] pinned_columns: Option<RwSignal<std::collections::HashMap<String, PinPosition>>>,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(DataTableRequest) -> Result<DataTableResponse<T>, String> + 'static,
{
    // 1. Estado central
    let state: DataTableState<T> = DataTableState::new();

    // 2. Props externos têm prioridade
    let column_widths_signal  = column_widths.unwrap_or(state.column_widths);
    let pinned_columns_signal = pinned_columns.unwrap_or(state.pinned_columns);

    // 3. Grupos de colunas reativos
    let cols_left = Signal::derive(move || {
        columns.with(|cols| {
            pinned_columns_signal.with(|pins| {
                cols.iter().filter(|c| pins.get(&c.id) == Some(&PinPosition::Left)).cloned().collect::<Vec<_>>()
            })
        })
    });
    let cols_center = Signal::derive(move || {
        columns.with(|cols| {
            pinned_columns_signal.with(|pins| {
                cols.iter().filter(|c| !matches!(pins.get(&c.id), Some(PinPosition::Left) | Some(PinPosition::Right))).cloned().collect::<Vec<_>>()
            })
        })
    });
    let cols_right = Signal::derive(move || {
        columns.with(|cols| {
            pinned_columns_signal.with(|pins| {
                cols.iter().filter(|c| pins.get(&c.id) == Some(&PinPosition::Right)).cloned().collect::<Vec<_>>()
            })
        })
    });

    // 4. Largura total das tabelas pinadas (para width fixo)
    let left_width = Signal::derive(move || {
        cols_left.with(|cols| {
            column_widths_signal.with(|widths| {
                cols.iter().map(|c| widths.get(&c.id).copied().unwrap_or(150)).sum::<u32>()
            })
        })
    });
    let right_width = Signal::derive(move || {
        cols_right.with(|cols| {
            column_widths_signal.with(|widths| {
                cols.iter().map(|c| widths.get(&c.id).copied().unwrap_or(150)).sum::<u32>()
            })
        })
    });

    // 5. Fetch
    let fetch_data = std::sync::Arc::new(fetch_data);
    let fetch_data_effect = fetch_data.clone();
    create_effect(move |_| {
        columns.track();
        state.current_page.track();
        state.sort_column.track();
        state.sort_ascending.track();
        state.filter_query.track();
        state.loading.set(true);
        state.error.set(None);
        let request = DataTableRequest {
            page:           state.current_page.get(),
            page_size,
            sort_column:    state.sort_column.get(),
            sort_ascending: state.sort_ascending.get(),
            filter_query:   state.filter_query.get(),
        };
        match fetch_data_effect(request) {
            Ok(r)  => { state.data.set(r.data); state.total.set(r.total); state.loading.set(false); }
            Err(e) => { state.error.set(Some(e)); state.loading.set(false); }
        }
    });

    // 6. Feature signals
    let table_id         = id.unwrap_or_else(|| "datatable-interactive".to_string());
    let table_id_clone   = table_id.clone();
    let density_signal   = density.unwrap_or_else(|| Signal::derive(|| "comfortable".to_string()));
    let zebra_signal     = zebra.unwrap_or_else(|| Signal::derive(|| false));
    let hover_signal     = row_hover.unwrap_or_else(|| Signal::derive(|| true));
    let sticky_signal    = sticky_header.unwrap_or_else(|| Signal::derive(|| false));
    let draggable_signal = draggable.unwrap_or_else(|| Signal::derive(|| false));
    let resizable_signal = resizable.unwrap_or_else(|| Signal::derive(|| false));
    let pinnable_signal  = pinnable.unwrap_or_else(|| Signal::derive(|| false));

    // 7. Hooks
    use super::data_table_column_resize::use_column_resize;
    use super::data_table_column_pin::use_column_pin;
    use super::data_table_column_reorder::use_column_reorder;

    use_column_resize(format!("{}-center-resize", table_id), resizable_signal, {
        let widths = column_widths_signal;
        move |col_id, width| { widths.update(|m| { m.insert(col_id, width); }); }
    });
    // limpar pins quando feature é desabilitada
    create_effect(move |_| {
        if !pinnable_signal.get() {
            pinned_columns_signal.update(|m| m.clear());
        }
    });

    use_column_pin(table_id.clone(), pinnable_signal, {
        let pins = pinned_columns_signal;
        move |col_id, position| { pins.update(|m| { m.insert(col_id, position); }); }
    });
    use_column_reorder(table_id.clone(), draggable_signal, move |from, to| {
        leptos::logging::log!("reorder: {} -> {}", from, to);
    });

    // 8. Render — 3 tabelas em flex
    view! {
        <div
            data-datatable=""
            data-density=move || density_signal.get()
            data-zebra=move || zebra_signal.get().to_string()
            data-row-hover=move || hover_signal.get().to_string()
            data-sticky-header=move || sticky_signal.get().to_string()
            data-resizable=move || resizable_signal.get().to_string()
            data-pinnable=move || pinnable_signal.get().to_string()
            data-draggable=move || draggable_signal.get().to_string()
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

            // Layout 3 tabelas
            <div data-datatable-scroll="" style="display: flex; overflow: hidden; border: var(--data-table-border-width, 1px) solid var(--data-table-border-color); border-radius: var(--data-table-radius);">

                // LEFT pinned
                <div
                    data-datatable-panel="left"
                    style=move || {
                        let w = left_width.get();
                        if w == 0 { "display: none".to_string() }
                        else { format!("width: {}px; flex-shrink: 0; overflow: hidden; border-right: 2px solid var(--data-table-border-color);", w) }
                    }
                >
                    <table data-datatable-table="" role="table" style="table-layout: fixed; border-collapse: collapse; width: 100%;">
                        <colgroup>
                            {move || cols_left.with(|cols| {
                                column_widths_signal.with(|widths| {
                                    cols.iter().map(|col| {
                                        let w = col_width_px(col, widths);
                                        view! { <col style=format!("width: {}", w) /> }
                                    }).collect_view()
                                })
                            })}
                        </colgroup>
                        <DataTableHeader<T>
                            columns=cols_left
                            sort_column=state.sort_column
                            sort_ascending=state.sort_ascending
                            current_page=state.current_page
                            draggable=Signal::derive(|| false)
                            resizable=Signal::derive(|| false)
                            pinnable=pinnable_signal
                            table_id=format!("{}-left", table_id_clone)
                            column_widths=column_widths_signal
                            panel="left"
                        />
                        <DataTableBody<T>
                            columns=cols_left
                            data=state.data
                            loading=state.loading
                            error=state.error
                        />
                    </table>
                </div>

                // CENTER scrollable
                <div
                    data-datatable-panel="center"
                    style=move || {
                        let max_h = if sticky_signal.get() { "max-height: 400px; ".to_string() } else { String::new() };
                        format!("{}flex: 1; overflow-x: auto; overflow-y: auto;", max_h)
                    }
                >
                    <table data-datatable-table="" role="table" style="table-layout: fixed; border-collapse: collapse;">
                        <colgroup>
                            {move || cols_center.with(|cols| {
                                column_widths_signal.with(|widths| {
                                    cols.iter().map(|col| {
                                        let w = col_width_px(col, widths);
                                        view! { <col style=format!("width: {}", w) /> }
                                    }).collect_view()
                                })
                            })}
                        </colgroup>
                        <DataTableHeader<T>
                            columns=cols_center
                            sort_column=state.sort_column
                            sort_ascending=state.sort_ascending
                            current_page=state.current_page
                            draggable=draggable_signal
                            resizable=resizable_signal
                            pinnable=pinnable_signal
                            table_id=format!("{}-center", table_id_clone)
                            column_widths=column_widths_signal
                            panel="center"
                        />
                        <DataTableBody<T>
                            columns=cols_center
                            data=state.data
                            loading=state.loading
                            error=state.error
                        />
                    </table>
                </div>

                // RIGHT pinned
                <div
                    data-datatable-panel="right"
                    style=move || {
                        let w = right_width.get();
                        if w == 0 { "display: none".to_string() }
                        else { format!("width: {}px; flex-shrink: 0; overflow: hidden; border-left: 2px solid var(--data-table-border-color);", w) }
                    }
                >
                    <table data-datatable-table="" role="table" style="table-layout: fixed; border-collapse: collapse; width: 100%;">
                        <colgroup>
                            {move || cols_right.with(|cols| {
                                column_widths_signal.with(|widths| {
                                    cols.iter().map(|col| {
                                        let w = col_width_px(col, widths);
                                        view! { <col style=format!("width: {}", w) /> }
                                    }).collect_view()
                                })
                            })}
                        </colgroup>
                        <DataTableHeader<T>
                            columns=cols_right
                            sort_column=state.sort_column
                            sort_ascending=state.sort_ascending
                            current_page=state.current_page
                            draggable=Signal::derive(|| false)
                            resizable=Signal::derive(|| false)
                            pinnable=pinnable_signal
                            table_id=format!("{}-right", table_id_clone)
                            column_widths=column_widths_signal
                            panel="right"
                        />
                        <DataTableBody<T>
                            columns=cols_right
                            data=state.data
                            loading=state.loading
                            error=state.error
                        />
                    </table>
                </div>

            </div>

            <DataTablePagination
                current_page=state.current_page
                total=state.total
                page_size=page_size
            />
        </div>
    }
}
