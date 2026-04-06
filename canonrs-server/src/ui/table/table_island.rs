use leptos::prelude::*;
use super::table_ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;
pub use super::table_ui::TableState;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TableIslandColumn {
    pub label: String,
}

#[island]
pub fn TableIsland(
    columns: Vec<TableIslandColumn>,
    rows: Vec<Vec<String>>,
    #[prop(optional)] striped: Option<bool>,
    #[prop(optional)] hoverable: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let striped   = striped.unwrap_or(false);
    let hoverable = hoverable.unwrap_or(false);
    let class     = class.unwrap_or_default();

    let header_cells = columns.iter().map(|col| {
        let label = col.label.clone();
        view! {
            <TableHead sort=SortDirection::None>
                {label}
            </TableHead>
        }
    }).collect::<Vec<_>>();

    let body_rows = rows.iter().map(|row| {
        let cells = row.iter().map(|val| {
            let val = val.clone();
            view! {
                <TableCell>{val}</TableCell>
            }
        }).collect::<Vec<_>>();
        view! {
            <TableRow selected=SelectionState::Unselected>
                {cells}
            </TableRow>
        }
    }).collect::<Vec<_>>();

    view! {
        <Table striped=striped hoverable=hoverable class=class>
            <TableHeader>
                <TableRow>
                    {header_cells}
                </TableRow>
            </TableHeader>
            <TableBody>
                {body_rows}
            </TableBody>
        </Table>
    }
}
