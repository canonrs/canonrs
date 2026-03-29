//! @canon-id: table
//! @canon-label: Table
//! @canon-family: data_display
//! @canon-category: Data
//! @canon-intent: Display tabular data
//! @canon-description: HTML table component
//! @canon-composable: true
//! @canon-capabilities: Selected
//! @canon-required-parts: TableHeader, TableBody, TableRow, TableHead, TableCell
//! @canon-optional-parts: TableFooter, TableCaption, TableWrapper
//! @canon-tags: table, tabular, rows, columns, data

use leptos::prelude::*;
use canonrs_core::primitives::{
    TableWrapperPrimitive, TablePrimitive, TableHeaderPrimitive,
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
        <TableWrapperPrimitive aria_label=aria_label.unwrap_or_default() class=class>
            <TablePrimitive state=state striped=striped hoverable=hoverable>
                {children.map(|c| c())}
            </TablePrimitive>
        </TableWrapperPrimitive>
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
) -> impl IntoView {
    view! {
        <TableRowPrimitive selected=selected class=class>
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
) -> impl IntoView {
    view! {
        <TableCellPrimitive class=class>
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
