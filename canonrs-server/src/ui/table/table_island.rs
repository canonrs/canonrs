//! @canon-level: strict
//! Table Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::table_ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;
pub use super::table_ui::TableState;



#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TableIslandColumn {
    pub label: String,
}

#[component]
pub fn TableIsland(
    children: Children,
    #[prop(default = false)] striped: bool,
    #[prop(default = false)] hoverable: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Table striped=striped hoverable=hoverable class=class>{children()}</Table>
    }
}

#[component]
pub fn TableHeaderIsland(children: Children) -> impl IntoView {
    view! { <TableHeader>{children()}</TableHeader> }
}

#[component]
pub fn TableBodyIsland(children: Children) -> impl IntoView {
    view! { <TableBody>{children()}</TableBody> }
}

#[component]
pub fn TableRowIsland(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
) -> impl IntoView {
    view! { <TableRow selected=selected>{children()}</TableRow> }
}

#[component]
pub fn TableHeadIsland(
    children: Children,
    #[prop(default = SortDirection::None)] sort: SortDirection,
) -> impl IntoView {
    view! { <TableHead sort=sort>{children()}</TableHead> }
}

#[component]
pub fn TableCellIsland(children: Children) -> impl IntoView {
    view! { <TableCell>{children()}</TableCell> }
}
