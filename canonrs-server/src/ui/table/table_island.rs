//! @canon-level: strict
//! Table Island — Canon Rule #340 (zero-logic passthrough)
//! CR-338 v3.0.0: init type — WASM-driven via canon-init-loader

use leptos::prelude::*;
use super::table_ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableFooter, TableCaption};
use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;
pub use super::table_ui::TableState;

#[component]
pub fn TableIsland(
    children: Children,
    #[prop(default = TableState::Idle)] state: TableState,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] hoverable: bool,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let aria_label = aria_label.unwrap_or_default();
    view! { <Table state=state striped=striped hoverable=hoverable aria_label=aria_label class=class>{children()}</Table> }
}

#[component]
pub fn TableHeaderIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableHeader class=class>{children()}</TableHeader> }
}

#[component]
pub fn TableBodyIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableBody class=class>{children()}</TableBody> }
}

#[component]
pub fn TableFooterIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableFooter class=class>{children()}</TableFooter> }
}

#[component]
pub fn TableRowIsland(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, optional)] href: Option<String>,
    #[prop(into, default = TextProp::from(""))] class: TextProp,
) -> impl IntoView {
    let href = href.unwrap_or_default();
    view! { <TableRow selected=selected href=href class=class>{children()}</TableRow> }
}

#[component]
pub fn TableHeadIsland(
    children: Children,
    #[prop(default = SortDirection::None)] sort: SortDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableHead sort=sort class=class>{children()}</TableHead> }
}

#[component]
pub fn TableCellIsland(
    children: Children,
    #[prop(default = false)] copyable: bool,
    #[prop(default = false)] truncate: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableCell copyable=copyable truncate=truncate class=class>{children()}</TableCell> }
}

#[component]
pub fn TableCaptionIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TableCaption class=class>{children()}</TableCaption> }
}
