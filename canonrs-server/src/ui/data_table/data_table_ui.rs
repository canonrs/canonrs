//! DataTable Full - HTML estático, comportamento delegado ao behavior JS

use leptos::prelude::*;
use std::sync::Arc;
use canonrs_core::primitives::{
    DataTablePrimitive, DataTableToolbarPrimitive, DataTableScrollPrimitive,
    DataTableTablePrimitive, DataTableHeadPrimitive, DataTableHeadRowPrimitive,
    DataTableHeadCellPrimitive, DataTableBodyPrimitive, DataTableRowPrimitive,
    DataTableCellPrimitive, DataTablePaginationPrimitive, DataTableEmptyPrimitive,
    DataTableDensity, SortDirection,
};
use crate::ui::dropdown_menu::dropdown_menu_island::{
    DropdownMenuIsland, DropdownMenuItemIsland,
};
#[derive(Clone)]
pub struct DataTableColumn<T> {
    pub key: String,
    pub label: String,
    pub render: std::sync::Arc<dyn Fn(&T) -> String + Send + Sync>,
}

impl<T> DataTableColumn<T> {
    pub fn new(key: impl Into<String>, label: impl Into<String>, render: impl Fn(&T) -> String + Send + Sync + 'static) -> Self {
        Self { key: key.into(), label: label.into(), render: std::sync::Arc::new(render) }
    }
}

#[derive(Clone)]
pub struct RowAction {
    pub id: &'static str,
    pub label: &'static str,
    pub danger: bool,
}

impl RowAction {
    pub fn new(id: &'static str, label: &'static str) -> Self {
        Self { id, label, danger: false }
    }
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

#[component]
pub fn DataTableStatic<T>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[prop(default = DataTableDensity::default())] density: DataTableDensity,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 10)] page_size: usize,
    #[prop(default = false)] selectable: bool,
    #[prop(into, default = String::new())] sync_chart: String,
    #[prop(into, default = String::new())] sync_scope: String,
    #[prop(default = false)] show_density: bool,
    #[prop(optional)] expand_render: Option<Arc<dyn Fn(&T) -> String + Send + Sync>>,
    #[prop(default = vec![])] row_actions: Vec<RowAction>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let total = data.len();
    let total_pages = ((total as f64) / (page_size as f64)).ceil().max(1.0) as usize;
    let col_count = columns.len()
        + if selectable { 1 } else { 0 }
        + if expand_render.is_some() { 1 } else { 0 }
        + if !row_actions.is_empty() { 1 } else { 0 };
    let visible_data = data.into_iter().enumerate().collect::<Vec<_>>();
    let cols = StoredValue::new(columns.clone());
    let expand_render = StoredValue::new(expand_render);
    let row_actions = StoredValue::new(row_actions);
    let initial_density = density.as_str();

    view! {
        <DataTablePrimitive
            density=density
            class=class
            attr:data-rs-page-size=page_size.to_string()
            attr:data-rs-current-page="1"
            attr:data-rs-total-pages=total_pages.to_string()
            attr:data-rs-selectable={selectable.then(|| "true")}
            attr:data-rs-chart-sync=sync_chart.clone()
            attr:data-rs-chart-sync-scope=sync_scope.clone()
        >
            <DataTableToolbarPrimitive>
                <input
                    type="text"
                    data-rs-datatable-filter=""
                    placeholder="Search..."
                />
                {show_density.then(|| view! {
                    <div data-rs-datatable-density-toggle="">
                        <button type="button" data-rs-density-btn="compact"
                            data-active={if initial_density == "compact" { "true" } else { "false" }}>
                            "Compact"
                        </button>
                        <button type="button" data-rs-density-btn="comfortable"
                            data-active={if initial_density == "comfortable" { "true" } else { "false" }}>
                            "Comfortable"
                        </button>
                        <button type="button" data-rs-density-btn="spacious"
                            data-active={if initial_density == "spacious" { "true" } else { "false" }}>
                            "Spacious"
                        </button>
                    </div>
                })}
                <DropdownMenuIsland trigger_label="Columns">
                    {columns.iter().enumerate().map(|(idx, col)| {
                        let label = col.label.clone();
                        view! {
                            <div
                                data-rs-dropdown-menu-checkbox-item=""
                                aria-checked="true"
                                data-rs-col-index=idx.to_string()
                            >
                                {label}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </DropdownMenuIsland>
            </DataTableToolbarPrimitive>

            <DataTableScrollPrimitive>
                <DataTableTablePrimitive>
                    <DataTableHeadPrimitive>
                        <DataTableHeadRowPrimitive>
                            {expand_render.get_value().is_some().then(|| view! {
                                <th data-rs-datatable-head-cell="" scope="col" data-rs-col-expand=""></th>
                            })}
                            {selectable.then(|| view! {
                                <th data-rs-datatable-head-cell="" scope="col" data-rs-col-select="">
                                    <input type="checkbox" data-rs-datatable-select-all="" />
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
                                        <span data-rs-datatable-head-label="">{label}</span>
                                        <span data-rs-datatable-sort-icon="" aria-hidden="true">"↕"</span>
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
                                        <td data-rs-datatable-cell="" data-rs-col-expand="">
                                            <button
                                                type="button"
                                                data-rs-datatable-expand-btn=""
                                                data-rs-row-id=idx.to_string()
                                                aria-expanded="false"
                                            >
                                                "▶"
                                            </button>
                                        </td>
                                    })}
                                    {selectable.then(|| view! {
                                        <td data-rs-datatable-cell="" data-rs-col-select="">
                                            <input type="checkbox" data-rs-datatable-select-row="" value=idx.to_string() />
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
                                    {(!row_actions.get_value().is_empty()).then(|| {
                                        let actions = row_actions.get_value();
                                        let row_id = idx.to_string();
                                        view! {
                                            <td data-rs-datatable-cell="" data-rs-col-actions="">
                                                <DropdownMenuIsland>
                                                    {actions.into_iter().map(|action| {
                                                        let rid = row_id.clone();
                                                        view! {
                                                            <DropdownMenuItemIsland
                                                                class={if action.danger { "danger".to_string() } else { String::new() }}
                                                            >
                                                                <span
                                                                    data-rs-datatable-action=action.id
                                                                    data-rs-row-id=rid
                                                                >{action.label}</span>
                                                            </DropdownMenuItemIsland>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </DropdownMenuIsland>
                                            </td>
                                        }
                                    })}
                                </DataTableRowPrimitive>
                            };

                            let expand_row = expand_content.map(|content| view! {
                                <tr
                                    data-rs-datatable-expand-row=""
                                    data-rs-row-id=idx.to_string()
                                    hidden=""
                                >
                                    <td
                                        data-rs-datatable-cell=""
                                        colspan=col_count.to_string()
                                    >
                                        <div data-rs-datatable-expand-content="">
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
                <button data-rs-action="prev" type="button" disabled=true>
                    "Previous"
                </button>
                <span data-rs-pagination-info="">
                    {format!("1 of {}", total_pages)}
                </span>
                <button type="button" data-rs-action="next" disabled={total_pages <= 1}>
                    "Next"
                </button>
            </DataTablePaginationPrimitive>
        </DataTablePrimitive>
    }
}

use std::collections::HashSet;

#[allow(unused_variables)]
#[component]
pub fn DataTableCore(
    columns: Vec<DataTableColumn<String>>,
    rows: Vec<Vec<String>>,
    visible_set: HashSet<usize>,
    sort_col: Option<usize>,
    sort_asc: bool,
    page: usize,
    total_pages: usize,
    hidden_cols: HashSet<usize>,
    density: &'static str,
    on_sort: Callback<usize>,
    on_prev: Callback<()>,
    on_next: Callback<()>,
    on_input: Callback<leptos::ev::Event>,
    on_col_toggle: Callback<usize>,
    on_density: Callback<&'static str>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();

    let header_cells = columns.iter().enumerate().map(|(i, col)| {
        let label = col.label.clone();
        let direction = if sort_col == Some(i) {
            if sort_asc { "▲" } else { "▼" }
        } else { "↕" };
        let is_hidden = hidden_cols.contains(&i);
        view! {
            <th
                data-rs-datatable-head-cell=""
                scope="col"
                data-rs-col-index=i.to_string()
                hidden=is_hidden
                style="cursor:pointer"
                on:click=move |_| on_sort.run(i)
            >
                <span data-rs-datatable-head-label="">{label}</span>
                <span data-rs-datatable-sort-icon="" aria-hidden="true">{direction}</span>
            </th>
        }
    }).collect::<Vec<_>>();

    let body_rows = rows.iter().enumerate().map(|(idx, row)| {
        let is_visible = visible_set.contains(&idx);
        let cells = row.iter().enumerate().map(|(ci, val)| {
            let val = val.clone();
            let is_col_hidden = hidden_cols.contains(&ci);
            view! {
                <td
                    data-rs-datatable-cell=""
                    data-rs-col-index=ci.to_string()
                    hidden=is_col_hidden
                >{val}</td>
            }
        }).collect::<Vec<_>>();
        view! {
            <tr
                data-rs-datatable-row=""
                data-rs-row-id=idx.to_string()
                data-rs-row-index=idx.to_string()
                hidden=!is_visible
            >
                {cells}
            </tr>
        }
    }).collect::<Vec<_>>();

    let col_toggles = columns.iter().enumerate().map(|(i, col)| {
        let label = col.label.clone();
        let is_hidden = hidden_cols.contains(&i);
        view! {
            <button
                type="button"
                data-rs-col-toggle=i.to_string()
                style=if is_hidden { "opacity:0.4" } else { "" }
                on:click=move |_| on_col_toggle.run(i)
            >{label}</button>
        }
    }).collect::<Vec<_>>();

    view! {
        <DataTablePrimitive
            class=class
            density=DataTableDensity::Comfortable
            attr:data-rs-density=density
        >
            <DataTableToolbarPrimitive>
                <input
                    type="text"
                    data-rs-datatable-filter=""
                    placeholder="Search..."
                    on:input=move |e| on_input.run(e)
                />
                <div style="display:flex;gap:4px;margin-left:auto">
                    <button type="button"
                        style=if density == "compact" { "font-weight:bold" } else { "" }
                        on:click=move |_| on_density.run("compact")
                    >"Compact"</button>
                    <button type="button"
                        style=if density == "comfortable" { "font-weight:bold" } else { "" }
                        on:click=move |_| on_density.run("comfortable")
                    >"Comfortable"</button>
                    <button type="button"
                        style=if density == "spacious" { "font-weight:bold" } else { "" }
                        on:click=move |_| on_density.run("spacious")
                    >"Spacious"</button>
                </div>
                <div style="display:flex;gap:4px">
                    {col_toggles}
                </div>
            </DataTableToolbarPrimitive>

            <DataTableScrollPrimitive>
                <DataTableTablePrimitive>
                    <DataTableHeadPrimitive>
                        <DataTableHeadRowPrimitive>
                            {header_cells}
                        </DataTableHeadRowPrimitive>
                    </DataTableHeadPrimitive>
                    <DataTableBodyPrimitive>
                        {body_rows}
                    </DataTableBodyPrimitive>
                </DataTableTablePrimitive>
            </DataTableScrollPrimitive>

            {visible_set.is_empty().then(|| view! {
                <DataTableEmptyPrimitive>
                    "No results found."
                </DataTableEmptyPrimitive>
            })}

            <DataTablePaginationPrimitive>
                <button
                    type="button"
                    data-rs-action="prev"
                    disabled=page <= 1
                    on:click=move |_| on_prev.run(())
                >"Previous"</button>
                <span data-rs-pagination-info="">
                    {format!("{} of {}", page, total_pages)}
                </span>
                <button
                    type="button"
                    data-rs-action="next"
                    disabled=page >= total_pages
                    on:click=move |_| on_next.run(())
                >"Next"</button>
            </DataTablePaginationPrimitive>
        </DataTablePrimitive>
    }
}
