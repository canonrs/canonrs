use leptos::prelude::*;
use super::table_island::{
    TableIsland, TableHeaderIsland, TableBodyIsland,
    TableRowIsland, TableHeadIsland, TableCellIsland,
};
use super::super::sheet::sheet_island::SheetIsland;
use canonrs_core::meta::SelectionState;

#[component]
pub fn TableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div data-rs-table-context="">
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
                            <TableRowIsland
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Alice"
                                attr:data-rs-meta="Engineer · Active · alice@very-long-domain-example.com"
                            >
                                <TableCellIsland copyable=true>"Alice"</TableCellIsland>
                                <TableCellIsland>"Engineer"</TableCellIsland>
                                <TableCellIsland>"Active"</TableCellIsland>
                                <TableCellIsland truncate=true>"alice@very-long-domain-example.com"</TableCellIsland>
                            </TableRowIsland>
                            <TableRowIsland
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Bob"
                                attr:data-rs-meta="Designer · Active · bob@another-very-long-domain-example.com"
                            >
                                <TableCellIsland copyable=true>"Bob"</TableCellIsland>
                                <TableCellIsland>"Designer"</TableCellIsland>
                                <TableCellIsland>"Active"</TableCellIsland>
                                <TableCellIsland truncate=true>"bob@another-very-long-domain-example.com"</TableCellIsland>
                            </TableRowIsland>
                            <TableRowIsland
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Carol"
                                attr:data-rs-meta="Manager · Away · carol@example.com"
                            >
                                <TableCellIsland copyable=true>"Carol"</TableCellIsland>
                                <TableCellIsland>"Manager"</TableCellIsland>
                                <TableCellIsland>"Away"</TableCellIsland>
                                <TableCellIsland truncate=true>"carol@example.com"</TableCellIsland>
                            </TableRowIsland>
                        </TableBodyIsland>
                    </TableIsland>
                    <SheetIsland
                        trigger_label=""
                        close_label="Close"
                    />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Click any row to open a detail sheet. Copy cells and truncation included."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Striped + selected"</span>
                <div data-rs-showcase-preview-row="">
                    <TableIsland striped=true hoverable=true>
                        <TableHeaderIsland>
                            <TableRowIsland>
                                <TableHeadIsland>"Name"</TableHeadIsland>
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
