use leptos::prelude::*;
use super::empty_table_ui::EmptyTable;
use crate::ui::table::{Table, TableHeader, TableBody, TableRow, TableHead};

#[component]
pub fn EmptyTableShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead><span>{"Name"}</span></TableHead>
                            <TableHead><span>{"Role"}</span></TableHead>
                            <TableHead><span>{"Status"}</span></TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <EmptyTable
                            colspan=3u32
                            title="No data available"
                            description="Add your first item to get started."
                        />
                    </TableBody>
                </Table>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Empty state rendered as valid table row with proper ARIA."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With action"</span>
                <div data-rs-showcase-preview-row="">
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead><span>{"Name"}</span></TableHead>
                                <TableHead><span>{"Email"}</span></TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <EmptyTable
                                colspan=2u32
                                title="No users found"
                                description="Invite your team members to get started."
                            />
                        </TableBody>
                    </Table>
                </div>
            </div>
        </div>
    }
}
