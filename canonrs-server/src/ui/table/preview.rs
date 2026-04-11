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
                // basic table
                <TableIsland hoverable=true>
                    <TableHeaderIsland>
                        <TableRowIsland>
                            <TableHeadIsland>"Name"</TableHeadIsland>
                            <TableHeadIsland>"Role"</TableHeadIsland>
                            <TableHeadIsland>"Status"</TableHeadIsland>
                            <TableHeadIsland>"Email"</TableHeadIsland>
                        </TableRowIsland>
                    </TableHeaderIsland>
                    <TableBodyIsland>
                        <TableRowIsland href="/users/alice">
                            <TableCellIsland copyable=true>"Alice"</TableCellIsland>
                            <TableCellIsland>"Engineer"</TableCellIsland>
                            <TableCellIsland>"Active"</TableCellIsland>
                            <TableCellIsland truncate=true>"alice@very-long-domain-example.com"</TableCellIsland>
                        </TableRowIsland>
                        <TableRowIsland href="/users/bob">
                            <TableCellIsland copyable=true>"Bob"</TableCellIsland>
                            <TableCellIsland>"Designer"</TableCellIsland>
                            <TableCellIsland>"Active"</TableCellIsland>
                            <TableCellIsland truncate=true>"bob@another-very-long-domain-example.com"</TableCellIsland>
                        </TableRowIsland>
                        <TableRowIsland href="/users/carol">
                            <TableCellIsland copyable=true>"Carol"</TableCellIsland>
                            <TableCellIsland>"Manager"</TableCellIsland>
                            <TableCellIsland>"Away"</TableCellIsland>
                            <TableCellIsland truncate=true>"carol@example.com"</TableCellIsland>
                        </TableRowIsland>
                    </TableBodyIsland>
                </TableIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Table with clickable rows, copy cells and truncation."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Striped + sorted"</span>
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
