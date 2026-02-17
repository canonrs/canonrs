//! DataTable Full - HTML estático, comportamento delegado ao behavior JS

use leptos::prelude::*;
use std::sync::Arc;
use crate::primitives::{
    DataTablePrimitive, DataTableToolbarPrimitive, DataTableScrollPrimitive,
    DataTableTablePrimitive, DataTableHeadPrimitive, DataTableHeadRowPrimitive,
    DataTableHeadCellPrimitive, DataTableBodyPrimitive, DataTableRowPrimitive,
    DataTableCellPrimitive, DataTablePaginationPrimitive, DataTableEmptyPrimitive,
    DataTableDensity, SortDirection,
};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuGroup,
};
use super::DataTableColumn;

#[component]
pub fn DataTableFull<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = DataTableDensity::default())] density: DataTableDensity,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 10)] page_size: usize,
    #[prop(default = false)] selectable: bool,
    #[prop(into, default = String::new())] sync_chart: String,
    #[prop(into, default = String::new())] sync_scope: String,
    #[prop(default = false)] show_density: bool,
    #[prop(optional)] expand_render: Option<Arc<dyn Fn(&T) -> String + Send + Sync>>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let total = data.len();
    let total_pages = ((total as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let col_count = columns.len()
        + if selectable { 1 } else { 0 }
        + if expand_render.is_some() { 1 } else { 0 };
    let visible_data = data.into_iter().enumerate().collect::<Vec<_>>();
    let cols = store_value(columns.clone());
    let expand_render = store_value(expand_render);
    let dropdown_id = format!("{}-columns", id);
    let trigger_id = format!("{}-columns-trigger", id);
    let initial_density = density.as_str();

    view! {
        <DataTablePrimitive
            density=density
            id=id.clone()
            class=class
            page_size=page_size.to_string()
            current_page="1".to_string()
            total_pages=total_pages.to_string()
            attr:data-selectable={selectable.then(|| "true")}
            sync_chart=sync_chart.clone()
            sync_scope=sync_scope.clone()
        >
            <DataTableToolbarPrimitive>
                <input
                    type="text"
                    data-datatable-filter=""
                    placeholder="Search..."
                />
                {show_density.then(|| view! {
                    <div data-datatable-density-toggle="">
                        <button type="button" data-density-btn="compact"
                            data-active={if initial_density == "compact" { "true" } else { "false" }}>
                            "Compact"
                        </button>
                        <button type="button" data-density-btn="comfortable"
                            data-active={if initial_density == "comfortable" { "true" } else { "false" }}>
                            "Comfortable"
                        </button>
                        <button type="button" data-density-btn="spacious"
                            data-active={if initial_density == "spacious" { "true" } else { "false" }}>
                            "Spacious"
                        </button>
                    </div>
                })}
                <DropdownMenu id=dropdown_id.clone()>
                    <DropdownMenuTrigger
                        target_dropdown_id=dropdown_id.clone()
                        id=trigger_id
                    >
                        "Columns ▾"
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <DropdownMenuGroup>
                            {columns.iter().enumerate().map(|(idx, col)| {
                                let label = col.label.clone();
                                view! {
                                    <div
                                        data-dropdown-menu-checkbox-item=""
                                        aria-checked="true"
                                        data-col-index=idx.to_string()
                                    >
                                        {label}
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </DropdownMenuGroup>
                    </DropdownMenuContent>
                </DropdownMenu>
            </DataTableToolbarPrimitive>

            <DataTableScrollPrimitive>
                <DataTableTablePrimitive>
                    <DataTableHeadPrimitive>
                        <DataTableHeadRowPrimitive>
                            {expand_render.get_value().is_some().then(|| view! {
                                <th data-datatable-head-cell="" scope="col" data-col-expand=""></th>
                            })}
                            {selectable.then(|| view! {
                                <th data-datatable-head-cell="" scope="col" data-col-select="">
                                    <input type="checkbox" data-datatable-select-all="" />
                                </th>
                            })}
                            {cols.get_value().into_iter().enumerate().map(|(idx, col)| {
                                let key = col.key.clone();
                                let label = col.label.clone();
                                view! {
                                    <DataTableHeadCellPrimitive
                                        sort_key=key
                                        sort_direction=SortDirection::None
                                        col_index=idx.to_string()
                                    >
                                        <span data-datatable-head-label="">{label}</span>
                                        <span data-datatable-sort-icon="" aria-hidden="true">"↕"</span>
                                    </DataTableHeadCellPrimitive>
                                }
                            }).collect::<Vec<_>>()}
                        </DataTableHeadRowPrimitive>
                    </DataTableHeadPrimitive>

                    <DataTableBodyPrimitive>
                        {visible_data.into_iter().flat_map(|(idx, row)| {
                            let row_cols = cols.get_value();
                            let expand_content = expand_render.get_value()
                                .as_ref()
                                .map(|f| f(&row));
                            let has_expand = expand_content.is_some();

                            let main_row = view! {
                                <DataTableRowPrimitive row_id=idx.to_string() row_index=idx>
                                    {has_expand.then(|| view! {
                                        <td data-datatable-cell="" data-col-expand="">
                                            <button
                                                type="button"
                                                data-datatable-expand-btn=""
                                                data-row-id=idx.to_string()
                                                aria-expanded="false"
                                            >
                                                "▶"
                                            </button>
                                        </td>
                                    })}
                                    {selectable.then(|| view! {
                                        <td data-datatable-cell="" data-col-select="">
                                            <input type="checkbox" data-datatable-select-row="" value=idx.to_string() />
                                        </td>
                                    })}
                                    {row_cols.into_iter().enumerate().map(|(col_idx, col)| {
                                        let value = (col.render)(&row);
                                        view! {
                                            <DataTableCellPrimitive col_index=col_idx.to_string()>
                                                {value}
                                            </DataTableCellPrimitive>
                                        }
                                    }).collect::<Vec<_>>()}
                                </DataTableRowPrimitive>
                            };

                            let expand_row = expand_content.map(|content| view! {
                                <tr
                                    data-datatable-expand-row=""
                                    data-row-id=idx.to_string()
                                    hidden=""
                                >
                                    <td
                                        data-datatable-cell=""
                                        colspan=col_count.to_string()
                                    >
                                        <div data-datatable-expand-content="">
                                            {content}
                                        </div>
                                    </td>
                                </tr>
                            });

                            vec![main_row.into_any(), expand_row.map(|v| v.into_any()).unwrap_or_else(|| view! { <></> }.into_any())]
                        }).collect::<Vec<_>>()}
                    </DataTableBodyPrimitive>
                </DataTableTablePrimitive>
            </DataTableScrollPrimitive>

            <DataTableEmptyPrimitive class="hidden".to_string()>
                "No results found."
            </DataTableEmptyPrimitive>

            <DataTablePaginationPrimitive>
                <button data-action="prev" type="button" disabled=true>
                    "Previous"
                </button>
                <span data-pagination-info="">
                    {format!("1 of {}", total_pages)}
                </span>
                <button type="button" data-action="next" disabled={total_pages <= 1}>
                    "Next"
                </button>
            </DataTablePaginationPrimitive>
        </DataTablePrimitive>
    }
}
