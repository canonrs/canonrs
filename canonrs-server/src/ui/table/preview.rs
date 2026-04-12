use leptos::prelude::*;
use super::table_boundary::{
    Table, TableHeader, TableBody,
    TableRow, TableHead, TableCell,
};
use super::super::sheet::Sheet;
use canonrs_core::meta::SelectionState;

#[component]
pub fn TableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
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
                                attr:data-rs-meta="Engineer · Active · alice@very-long-domain-example.com"
                            >
                                <TableCell copyable=true>"Alice"</TableCell>
                                <TableCell>"Engineer"</TableCell>
                                <TableCell>"Active"</TableCell>
                                <TableCell truncate=true>"alice@very-long-domain-example.com"</TableCell>
                            </TableRow>
                            <TableRow
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Bob"
                                attr:data-rs-meta="Designer · Active · bob@another-very-long-domain-example.com"
                            >
                                <TableCell copyable=true>"Bob"</TableCell>
                                <TableCell>"Designer"</TableCell>
                                <TableCell>"Active"</TableCell>
                                <TableCell truncate=true>"bob@another-very-long-domain-example.com"</TableCell>
                            </TableRow>
                            <TableRow
                                attr:data-rs-action="open-sheet"
                                attr:data-rs-label="Carol"
                                attr:data-rs-meta="Manager · Away · carol@example.com"
                            >
                                <TableCell copyable=true>"Carol"</TableCell>
                                <TableCell>"Manager"</TableCell>
                                <TableCell>"Away"</TableCell>
                                <TableCell truncate=true>"carol@example.com"</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                    <Sheet
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
                </div>
            </div>
        </div>
    }
}
