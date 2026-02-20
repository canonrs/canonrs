//! # DataTable Block
//! Enhanced table with pagination, selection, sorting

use leptos::prelude::*;

#[component]
pub fn DataTableBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] pagination: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("canon-data-table {}", class)
            attr:data-block="data-table"
        >
            <div class="canon-data-table__wrapper">
                <table class="canon-data-table__table">
                    {header.map(|h| view! {
                        <thead class="canon-data-table__head">{h()}</thead>
                    })}
                    <tbody class="canon-data-table__body">
                        {children()}
                    </tbody>
                </table>
            </div>

            {pagination.map(|p| view! {
                <div class="canon-data-table__pagination">{p()}</div>
            })}
        </div>
    }
}

#[component]
pub fn DataTableRow(
    #[prop(default = false)] selected: bool,
    #[prop(optional)] on_select: Option<Callback<bool>>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            class=move || format!(
                "canon-data-table__row {} {}",
                if selected { "canon-data-table__row--selected" } else { "" },
                class
            )
            attr:data-action="select-row" on:click=move |_| {
                if let Some(cb) = on_select {
                    cb.run(!selected);
                }
            }
        >
            {children()}
        </tr>
    }
}

#[component]
pub fn DataTableCell(
    #[prop(default = false)] header: bool,
    #[prop(default = false)] sortable: bool,
    #[prop(optional)] on_sort: Option<Callback<()>>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    if header {
        view! {
            <th
                class=format!("canon-data-table__cell canon-data-table__cell--header {} {}", if sortable { "canon-data-table__cell--sortable" } else { "" }, class)
                attr:data-action="sort-column" on:click=move |_| {
                    if sortable {
                        if let Some(cb) = on_sort {
                            cb.run(());
                        }
                    }
                }
            >
                {children()}
                {sortable.then(|| view! {
                    <span class="canon-data-table__sort-icon">"⇅"</span>
                })}
            </th>
        }.into_any()
    } else {
        view! {
            <td class=format!("canon-data-table__cell {}", class)>
                {children()}
            </td>
        }.into_any()
    }
}
