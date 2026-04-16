#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    TablePrimitive, TableHeaderPrimitive,
    TableBodyPrimitive, TableFooterPrimitive, TableRowPrimitive,
    TableHeadPrimitive, TableCellPrimitive, TableCaptionPrimitive,
    SortDirection,
};
use canonrs_core::meta::SelectionState;
pub use canonrs_core::primitives::TableState;

#[component]
pub fn Table(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = TableState::Idle)] state: TableState,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] hoverable: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TablePrimitive state=state striped=striped hoverable=hoverable aria_label=aria_label.unwrap_or_default() class=class>
            {children.map(|c| c())}
        </TablePrimitive>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableHeaderPrimitive class=class>
            {children.map(|c| c())}
        </TableHeaderPrimitive>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableBodyPrimitive class=class>
            {children.map(|c| c())}
        </TableBodyPrimitive>
    }
}

#[component]
pub fn TableFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableFooterPrimitive class=class>
            {children.map(|c| c())}
        </TableFooterPrimitive>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, default = TextProp::from(""))] class: TextProp,
    #[prop(into, optional)] href: Option<String>,
    #[prop(into, optional)] row_action: Option<String>,
    #[prop(into, optional)] row_label: Option<String>,
    #[prop(into, optional)] row_meta: Option<String>,
) -> impl IntoView {
    let href = href.unwrap_or_default();
    let row_action = row_action.unwrap_or_default();
    let row_label = row_label.unwrap_or_default();
    let row_meta = row_meta.unwrap_or_default();
    view! {
        <TableRowPrimitive selected=selected class=class href=href row_action=row_action row_label=row_label row_meta=row_meta>
            {children.map(|c| c())}
        </TableRowPrimitive>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = SortDirection::None)] sort: SortDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableHeadPrimitive sort=sort class=class>
            {children.map(|c| c())}
        </TableHeadPrimitive>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] copyable: bool,
    #[prop(default = false)] truncate: bool,
) -> impl IntoView {
    view! {
        <TableCellPrimitive class=class copyable=copyable truncate=truncate>
            {children.map(|c| c())}
        </TableCellPrimitive>
    }
}

#[component]
pub fn TableCaption(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableCaptionPrimitive class=class>
            {children.map(|c| c())}
        </TableCaptionPrimitive>
    }
}
