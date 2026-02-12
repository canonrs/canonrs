use leptos::prelude::*;

#[component]
pub fn DataTablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = MaybeSignal::Static(String::from("comfortable")))] density: MaybeSignal<String>,
) -> impl IntoView {
    view! {
        <div
            data-datatable=""
            data-density=move || density.get()
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableToolbarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-toolbar="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableScrollPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-scroll="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableTablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <table data-datatable-table="" class=class role="table">
            {children.map(|c| c())}
        </table>
    }
}

#[component]
pub fn DataTableHeadPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <thead data-datatable-head="" class=class>
            {children.map(|c| c())}
        </thead>
    }
}

#[component]
pub fn DataTableHeadRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-datatable-head-row="" class=class role="row">
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn DataTableHeadCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] sort_key: String,
) -> impl IntoView {
    let sortable = !sort_key.is_empty();
    view! {
        <th
            data-datatable-head-cell=""
            data-sort-key=sort_key
            data-sortable=sortable
            aria-sort="none"
            role="columnheader"
            class=class
        >
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn DataTableBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tbody data-datatable-body="" class=class>
            {children.map(|c| c())}
        </tbody>
    }
}

#[component]
pub fn DataTableRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(default = false)] selected: bool,
) -> impl IntoView {
    let row_id = if !id.is_empty() && !id.starts_with("row-") {
        format!("row-{}", id)
    } else if !id.is_empty() {
        id.clone()
    } else {
        String::new()
    };

    view! {
        <tr
            data-datatable-row=""
            data-row-id=id
            data-selected=selected
            role="row"
            class=class
            id=row_id
        >
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn DataTableCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <td data-datatable-cell="" role="cell" class=class>
            {children.map(|c| c())}
        </td>
    }
}

#[component]
pub fn DataTableFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tfoot data-datatable-footer="" class=class>
            {children.map(|c| c())}
        </tfoot>
    }
}

#[component]
pub fn DataTablePaginationPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-pagination="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableEmptyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-empty="" data-empty="true" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DataTableLoadingPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-datatable-loading="" data-loading="true" class=class>
            {children.map(|c| c())}
        </div>
    }
}
