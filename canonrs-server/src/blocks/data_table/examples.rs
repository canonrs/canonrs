use leptos::prelude::*;
use super::{DataTableBlock, DataTableRow, DataTableCell};

pub fn basic_example() -> impl IntoView {
    view! {
        <DataTableBlock
            header=leptos::children::ToChildren::to_children(|| view!{
                <tr>
                    <DataTableCell header=true>"Name"</DataTableCell>
                    <DataTableCell header=true>"Status"</DataTableCell>
                    <DataTableCell header=true>"Date"</DataTableCell>
                </tr>
            })
        >
            <DataTableRow>
                <DataTableCell>"Alice"</DataTableCell>
                <DataTableCell>"Active"</DataTableCell>
                <DataTableCell>"2024-01-01"</DataTableCell>
            </DataTableRow>
            <DataTableRow>
                <DataTableCell>"Bob"</DataTableCell>
                <DataTableCell>"Inactive"</DataTableCell>
                <DataTableCell>"2024-01-02"</DataTableCell>
            </DataTableRow>
        </DataTableBlock>
    }
}
