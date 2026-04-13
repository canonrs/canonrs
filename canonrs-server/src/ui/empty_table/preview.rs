use leptos::prelude::*;
use super::empty_table_boundary::EmptyTable;
use crate::blocks::data_table::DataTableBlock;
use crate::ui::table::table_boundary::{Table, TableHeader, TableBody, TableRow, TableHead};
use canonrs_core::slot;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn EmptyTableShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <DataTableBlock
                empty=slot!(|| view! {
                    <EmptyTable
                        colspan=3u32
                        title="No data available"
                        description="Add your first item to get started."
                    />
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "Empty state rendered as valid table row with proper ARIA."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Inside table"</span>
                <DataTableBlock
                    header=slot!(|| view! {
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead><span>"Name"</span></TableHead>
                                    <TableHead><span>"Email"</span></TableHead>
                                </TableRow>
                            </TableHeader>
                        </Table>
                    }.into_any())
                    empty=slot!(|| view! {
                        <Table>
                            <TableBody>
                                <EmptyTable
                                    colspan=2u32
                                    title="No users found"
                                    description="Invite your team members to get started."
                                />
                            </TableBody>
                        </Table>
                    }.into_any())
                />
            </Stack>
        </Stack>
    }
}
