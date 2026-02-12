use leptos::prelude::*;
use crate::primitives::{
    DataTablePrimitive, DataTableToolbarPrimitive, DataTableScrollPrimitive,
    DataTableTablePrimitive, DataTableHeadPrimitive, DataTableHeadRowPrimitive,
    DataTableHeadCellPrimitive, DataTableBodyPrimitive, DataTableRowPrimitive,
    DataTableCellPrimitive, DataTableFooterPrimitive, DataTablePaginationPrimitive,
    DataTableEmptyPrimitive, DataTableLoadingPrimitive
};

#[component]
pub fn DataTable(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, default = String::new())] density: String,
) -> impl IntoView {
    view! {
        <DataTablePrimitive
            class=class
            id=id.unwrap_or_default()
            density=density
        >
            {children.map(|c| c())}
        </DataTablePrimitive>
    }
}

#[component]
pub fn DataTableToolbar(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableToolbarPrimitive class=class>
            {children.map(|c| c())}
        </DataTableToolbarPrimitive>
    }
}

#[component]
pub fn DataTableScroll(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableScrollPrimitive class=class>
            {children.map(|c| c())}
        </DataTableScrollPrimitive>
    }
}

#[component]
pub fn DataTableTable(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableTablePrimitive class=class>
            {children.map(|c| c())}
        </DataTableTablePrimitive>
    }
}

#[component]
pub fn DataTableHead(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableHeadPrimitive class=class>
            {children.map(|c| c())}
        </DataTableHeadPrimitive>
    }
}

#[component]
pub fn DataTableHeadRow(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableHeadRowPrimitive class=class>
            {children.map(|c| c())}
        </DataTableHeadRowPrimitive>
    }
}

#[component]
pub fn DataTableHeadCell(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, default = String::new())] sort_key: String,
) -> impl IntoView {
    view! {
        <DataTableHeadCellPrimitive class=class sort_key=sort_key>
            {children.map(|c| c())}
        </DataTableHeadCellPrimitive>
    }
}

#[component]
pub fn DataTableBody(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableBodyPrimitive class=class>
            {children.map(|c| c())}
        </DataTableBodyPrimitive>
    }
}

#[component]
pub fn DataTableRow(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(default = false)] selected: bool,
) -> impl IntoView {
    view! {
        <DataTableRowPrimitive class=class id=id selected=selected>
            {children.map(|c| c())}
        </DataTableRowPrimitive>
    }
}

#[component]
pub fn DataTableCell(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableCellPrimitive class=class>
            {children.map(|c| c())}
        </DataTableCellPrimitive>
    }
}

#[component]
pub fn DataTableFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableFooterPrimitive class=class>
            {children.map(|c| c())}
        </DataTableFooterPrimitive>
    }
}

#[component]
pub fn DataTablePagination(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTablePaginationPrimitive class=class>
            {children.map(|c| c())}
        </DataTablePaginationPrimitive>
    }
}

#[component]
pub fn DataTableEmpty(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableEmptyPrimitive class=class>
            {children.map(|c| c())}
        </DataTableEmptyPrimitive>
    }
}

#[component]
pub fn DataTableLoading(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DataTableLoadingPrimitive class=class>
            {children.map(|c| c())}
        </DataTableLoadingPrimitive>
    }
}
