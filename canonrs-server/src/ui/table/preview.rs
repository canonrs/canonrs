use leptos::prelude::*;
use super::table_boundary::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
use crate::blocks::data_table::DataTableBlock;
use crate::ui::sheet::sheet_boundary::Sheet;
use canonrs_core::slot;
use canonrs_core::meta::SelectionState;

#[component]
pub fn TableShowcasePreview() -> impl IntoView {
    view! {
        <DataTableBlock
            body=slot!(|| view! {
                <div data-rs-table-context="" data-rs-interaction="data">
                    <Table hoverable=true>
                        <TableHeader>
                            <TableRow>
                                <TableHead>"Name"</TableHead>
                                <TableHead>"Role"</TableHead>
                                <TableHead>"Status"</TableHead>
                                <TableHead>"Email"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Alice"
                                attr:data-rs-meta="Engineer · Active · alice@example.com"
                            >
                                <TableCell copyable=true>"Alice"</TableCell>
                                <TableCell>"Engineer"</TableCell>
                                <TableCell>"Active"</TableCell>
                                <TableCell truncate=true>"alice@example.com"</TableCell>
                            </TableRow>
                            <TableRow
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Bob"
                                attr:data-rs-meta="Designer · Active · bob@example.com"
                            >
                                <TableCell copyable=true>"Bob"</TableCell>
                                <TableCell>"Designer"</TableCell>
                                <TableCell>"Active"</TableCell>
                                <TableCell truncate=true>"bob@example.com"</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                    <Sheet trigger_label="" close_label="Close" />
                </div>
            }.into_any())
        />
        <p data-rs-showcase-preview-anchor="">
            "Click any row to open a detail sheet. Copy cells and truncation included."
        </p>
        <DataTableBlock
            body=slot!(|| view! {
                <Table striped=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHead>"Name"</TableHead>
                            <TableHead>"Score"</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Alice"</TableCell>
                            <TableCell>"98"</TableCell>
                        </TableRow>
                        <TableRow selected=SelectionState::Selected>
                            <TableCell>"Bob"</TableCell>
                            <TableCell>"87"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            }.into_any())
        />
    }
}
