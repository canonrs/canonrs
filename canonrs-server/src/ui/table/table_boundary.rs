//! @canon-level: strict
//! Table Boundary — Canon Rule #340 (zero-logic passthrough)

use leptos::prelude::*;
use super::table_ui::{
    Table as TableUi,
    TableHeader as TableHeaderUi,
    TableBody as TableBodyUi,
    TableRow as TableRowUi,
    TableHead as TableHeadUi,
    TableCell as TableCellUi,
    TableFooter as TableFooterUi,
    TableCaption as TableCaptionUi,
};
pub use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;
pub use super::table_ui::TableState;

#[component]
pub fn Table(
    children: Children,
    #[prop(default = TableState::Idle)] state: TableState,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] hoverable: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableUi state=state striped=striped hoverable=hoverable aria_label=aria_label.unwrap_or_default() class=class>{children()}</TableUi> }
}

#[component]
pub fn TableHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableHeaderUi class=class>{children()}</TableHeaderUi> }
}

#[component]
pub fn TableBody(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableBodyUi class=class>{children()}</TableBodyUi> }
}

#[component]
pub fn TableFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableFooterUi class=class>{children()}</TableFooterUi> }
}

#[component]
pub fn TableRow(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, optional)] href: Option<String>,
    #[prop(into, default = TextProp::from(""))] class: TextProp,
) -> impl IntoView {
    let href = href.unwrap_or_default();
    view! { <TableRowUi selected=selected href=href class=class>{children()}</TableRowUi> }
}

#[component]
pub fn TableHead(
    children: Children,
    #[prop(default = SortDirection::None)] sort: SortDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableHeadUi sort=sort class=class>{children()}</TableHeadUi> }
}

#[component]
pub fn TableCell(
    children: Children,
    #[prop(default = false)] copyable: bool,
    #[prop(default = false)] truncate: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableCellUi copyable=copyable truncate=truncate class=class>{children()}</TableCellUi> }
}

#[component]
pub fn TableCaption(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableCaptionUi class=class>{children()}</TableCaptionUi> }
}
