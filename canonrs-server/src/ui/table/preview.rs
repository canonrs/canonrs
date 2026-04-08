use leptos::prelude::*;
use super::table_island::{
    TableIsland, TableHeaderIsland, TableBodyIsland,
    TableRowIsland, TableHeadIsland, TableCellIsland,
};
use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;

#[component]
pub fn TableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TableIsland>
                    <TableHeaderIsland>
                        <TableRowIsland>
                            <TableHeadIsland>"Name"</TableHeadIsland>
                            <TableHeadIsland>"Role"</TableHeadIsland>
                            <TableHeadIsland>"Status"</TableHeadIsland>
                        </TableRowIsland>
                    </TableHeaderIsland>
                    <TableBodyIsland>
                        <TableRowIsland>
                            <TableCellIsland>"Alice"</TableCellIsland>
                            <TableCellIsland>"Engineer"</TableCellIsland>
                            <TableCellIsland>"Active"</TableCellIsland>
                        </TableRowIsland>
                        <TableRowIsland>
                            <TableCellIsland>"Bob"</TableCellIsland>
                            <TableCellIsland>"Designer"</TableCellIsland>
                            <TableCellIsland>"Active"</TableCellIsland>
                        </TableRowIsland>
                        <TableRowIsland>
                            <TableCellIsland>"Carol"</TableCellIsland>
                            <TableCellIsland>"Manager"</TableCellIsland>
                            <TableCellIsland>"Away"</TableCellIsland>
                        </TableRowIsland>
                    </TableBodyIsland>
                </TableIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Table state, sorting and selection enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Striped + hoverable"</span>
                <div data-rs-showcase-preview-row="">
                    <TableIsland striped=true hoverable=true>
                        <TableHeaderIsland>
                            <TableRowIsland>
                                <TableHeadIsland sort=SortDirection::Ascending>"Name"</TableHeadIsland>
                                <TableHeadIsland>"Score"</TableHeadIsland>
                            </TableRowIsland>
                        </TableHeaderIsland>
                        <TableBodyIsland>
                            <TableRowIsland>
                                <TableCellIsland>"Alice"</TableCellIsland>
                                <TableCellIsland>"98"</TableCellIsland>
                            </TableRowIsland>
                            <TableRowIsland selected=SelectionState::Selected>
                                <TableCellIsland>"Bob"</TableCellIsland>
                                <TableCellIsland>"87"</TableCellIsland>
                            </TableRowIsland>
                        </TableBodyIsland>
                    </TableIsland>
                </div>
            </div>
        </div>
    }
}
