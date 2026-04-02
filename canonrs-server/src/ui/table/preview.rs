use leptos::prelude::*;
use super::table_ui::{
    Table, TableHeader, TableBody, TableRow,
    TableHead, TableCell, TableCaption,
};
use canonrs_core::primitives::SortDirection;
use canonrs_core::meta::SelectionState;

#[component]
pub fn TableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Table aria_label="User table">
                    <TableCaption><span>{"Team members"}</span></TableCaption>
                    <TableHeader>
                        <TableRow>
                            <TableHead><span>{"Name"}</span></TableHead>
                            <TableHead><span>{"Role"}</span></TableHead>
                            <TableHead><span>{"Status"}</span></TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><span>{"Alice"}</span></TableCell>
                            <TableCell><span>{"Engineer"}</span></TableCell>
                            <TableCell><span>{"Active"}</span></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><span>{"Bob"}</span></TableCell>
                            <TableCell><span>{"Designer"}</span></TableCell>
                            <TableCell><span>{"Active"}</span></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><span>{"Carol"}</span></TableCell>
                            <TableCell><span>{"Manager"}</span></TableCell>
                            <TableCell><span>{"Away"}</span></TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
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
