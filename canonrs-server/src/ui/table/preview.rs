use leptos::prelude::*;
use super::table_ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
use super::table_island::{TableIsland, TableIslandColumn};
use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;

#[component]
pub fn TableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TableIsland
                    columns=vec![
                        TableIslandColumn { label: "Name".to_string() },
                        TableIslandColumn { label: "Role".to_string() },
                        TableIslandColumn { label: "Status".to_string() },
                    ]
                    rows=vec![
                        vec!["Alice".to_string(), "Engineer".to_string(), "Active".to_string()],
                        vec!["Bob".to_string(),   "Designer".to_string(), "Active".to_string()],
                        vec!["Carol".to_string(), "Manager".to_string(),  "Away".to_string()],
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Table state, sorting and selection enforced structurally."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Striped + hoverable"</span>
                <div data-rs-showcase-preview-row="">
                    <Table striped=true hoverable=true>
                        <TableHeader>
                            <TableRow>
                                <TableHead sort=SortDirection::Ascending><span>{"Name"}</span></TableHead>
                                <TableHead><span>{"Score"}</span></TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow>
                                <TableCell><span>{"Alice"}</span></TableCell>
                                <TableCell><span>{"98"}</span></TableCell>
                            </TableRow>
                            <TableRow selected=SelectionState::Selected>
                                <TableCell><span>{"Bob"}</span></TableCell>
                                <TableCell><span>{"87"}</span></TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><span>{"Carol"}</span></TableCell>
                                <TableCell><span>{"76"}</span></TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </div>
            </div>

        </div>
    }
}
