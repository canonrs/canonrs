use leptos::prelude::*;
use std::collections::HashSet;
use canonrs_core::primitives::{
    DataTablePrimitive, DataTableToolbarPrimitive, DataTableScrollPrimitive,
    DataTableTablePrimitive, DataTableHeadPrimitive, DataTableHeadRowPrimitive,
    DataTableBodyPrimitive, DataTablePaginationPrimitive, DataTableEmptyPrimitive,
    DataTableDensity,
};

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataTableIslandColumn {
    pub key:   String,
    pub label: String,
}

#[allow(unused_variables)]
#[island]
pub fn DataTableIsland(
    columns: Vec<DataTableIslandColumn>,
    rows: Vec<Vec<String>>,
    #[prop(optional)] page_size: Option<usize>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let page_size = page_size.unwrap_or(10);
    let class     = class.unwrap_or_default();

    let query      = RwSignal::new(String::new());
    let sort_col   = RwSignal::new(Option::<usize>::None);
    let sort_asc   = RwSignal::new(true);
    let page       = RwSignal::new(1usize);
    let density    = RwSignal::new("comfortable");
    let hidden_cols: RwSignal<HashSet<usize>> = RwSignal::new(HashSet::new());

    let stored_rows = StoredValue::new(rows.clone());

    let total_pages = Memo::new(move |_| {
        let q = query.get().to_lowercase();
        let visible = stored_rows.get_value().iter()
            .filter(|row| q.is_empty() || row.iter().any(|c| c.to_lowercase().contains(&q)))
            .count();
        ((visible as f64) / (page_size as f64)).ceil().max(1.0) as usize
    });

    let visible_set = Memo::new(move |_| -> HashSet<usize> {
        let q = query.get().to_lowercase();
        let sc = sort_col.get();
        let sa = sort_asc.get();
        let p  = page.get();
        let rows_val = stored_rows.get_value();
        let mut filtered: Vec<usize> = rows_val.iter().enumerate()
            .filter(|(_, row)| q.is_empty() || row.iter().any(|c| c.to_lowercase().contains(&q)))
            .map(|(i, _)| i)
            .collect();
        if let Some(col) = sc {
            filtered.sort_by(|&a, &b| {
                let va = rows_val[a].get(col).map(|s| s.as_str()).unwrap_or("");
                let vb = rows_val[b].get(col).map(|s| s.as_str()).unwrap_or("");
                if sa { va.cmp(vb) } else { vb.cmp(va) }
            });
        }
        let start = (p - 1) * page_size;
        filtered.into_iter().skip(start).take(page_size).collect()
    });

    let on_input = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        let val = e.target()
            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
            .map(|el| el.value())
            .unwrap_or_default();
        query.set(val);
        page.set(1);
    };

    let on_prev = move |_: leptos::ev::MouseEvent| {
        if page.get_untracked() > 1 { page.update(|p| *p -= 1); }
    };

    let on_next = move |_: leptos::ev::MouseEvent| {
        let tp = total_pages.get_untracked();
        if page.get_untracked() < tp { page.update(|p| *p += 1); }
    };

    let header_cells = columns.iter().enumerate().map(|(i, col)| {
        let label = col.label.clone();
        let on_click = move |_: leptos::ev::MouseEvent| {
            if sort_col.get_untracked() == Some(i) {
                if sort_asc.get_untracked() { sort_asc.set(false); }
                else { sort_col.set(None); sort_asc.set(true); }
            } else {
                sort_col.set(Some(i));
                sort_asc.set(true);
            }
            page.set(1);
        };
        let sort_icon = move || {
            if sort_col.get() == Some(i) {
                if sort_asc.get() { "▲" } else { "▼" }
            } else { "↕" }
        };
        view! {
            <th
                data-rs-datatable-head-cell=""
                scope="col"
                data-rs-col-index=i.to_string()
                prop:hidden=move || hidden_cols.get().contains(&i)
                style="cursor:pointer"
                on:click=on_click
            >
                <span data-rs-datatable-head-label="">{label}</span>
                <span data-rs-datatable-sort-icon="" aria-hidden="true">{sort_icon}</span>
            </th>
        }
    }).collect::<Vec<_>>();

    let body_rows = rows.iter().enumerate().map(|(idx, row)| {
        let cells = row.iter().enumerate().map(|(ci, val)| {
            let val = val.clone();
            view! {
                <td
                    data-rs-datatable-cell=""
                    data-rs-col-index=ci.to_string()
                    prop:hidden=move || hidden_cols.get().contains(&ci)
                >{val}</td>
            }
        }).collect::<Vec<_>>();
        view! {
            <tr
                data-rs-datatable-row=""
                data-rs-row-id=idx.to_string()
                data-rs-row-index=idx.to_string()
                prop:hidden=move || !visible_set.get().contains(&idx)
            >
                {cells}
            </tr>
        }
    }).collect::<Vec<_>>();

    let col_dropdown_open = RwSignal::new(false);

    let col_items = columns.iter().enumerate().map(|(i, col)| {
        let label = col.label.clone();
        let on_toggle = move |_: leptos::ev::MouseEvent| {
            hidden_cols.update(|h| {
                if h.contains(&i) { h.remove(&i); } else { h.insert(i); }
            });
        };
        view! {
            <div
                data-rs-dropdown-menu-item=""
                role="menuitem"
                on:click=on_toggle
            >
                <span prop:style=move || if hidden_cols.get().contains(&i) { "opacity:0.4;margin-right:6px" } else { "margin-right:6px" }>
                    {move || if hidden_cols.get().contains(&i) { "☐" } else { "☑" }}
                </span>
                {label}
            </div>
        }
    }).collect::<Vec<_>>();

    let on_col_trigger = move |e: leptos::ev::MouseEvent| {
        e.stop_propagation();
        col_dropdown_open.update(|v| *v = !*v);
    };

    view! {
        <DataTablePrimitive
            class=class
            density=DataTableDensity::Comfortable
            attr:data-rs-density=move || density.get()
        >
            <DataTableToolbarPrimitive>
                <input
                    type="text"
                    data-rs-datatable-filter=""
                    placeholder="Search..."
                    on:input=on_input
                />
                <div style="display:flex;gap:4px;margin-left:auto">
                    <button type="button"
                        prop:style=move || if density.get() == "compact" { "font-weight:bold" } else { "" }
                        on:click=move |_| density.set("compact")
                    >"Compact"</button>
                    <button type="button"
                        prop:style=move || if density.get() == "comfortable" { "font-weight:bold" } else { "" }
                        on:click=move |_| density.set("comfortable")
                    >"Comfortable"</button>
                    <button type="button"
                        prop:style=move || if density.get() == "spacious" { "font-weight:bold" } else { "" }
                        on:click=move |_| density.set("spacious")
                    >"Spacious"</button>
                </div>
                <div
                    data-rs-dropdown-menu=""
                    data-rs-state=move || if col_dropdown_open.get() { "open" } else { "closed" }
                >
                    <button
                        type="button"
                        data-rs-dropdown-menu-trigger=""
                        on:click=on_col_trigger
                    >
                        "Columns"
                    </button>
                    <div
                        data-rs-dropdown-menu-content=""
                        prop:hidden=move || !col_dropdown_open.get()
                        role="menu"
                    >
                        {col_items}
                    </div>
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

            {move || visible_set.get().is_empty().then(|| view! {
                <DataTableEmptyPrimitive>
                    "No results found."
                </DataTableEmptyPrimitive>
            })}

            <DataTablePaginationPrimitive>
                <button type="button" data-rs-action="prev" on:click=on_prev>
                    "Previous"
                </button>
                <span data-rs-pagination-info="">
                    {move || format!("{} of {}", page.get(), total_pages.get())}
                </span>
                <button type="button" data-rs-action="next" on:click=on_next>
                    "Next"
                </button>
            </DataTablePaginationPrimitive>
        </DataTablePrimitive>
    }
}
