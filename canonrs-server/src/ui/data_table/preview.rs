use leptos::prelude::*;
use super::data_table_boundary::DataTable;
use crate::blocks::data_table::DataTableBlock;
use canonrs_core::slot;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn DataTableStaticShowcasePreview() -> impl IntoView {
    let data = vec![
        vec!["Alice".to_string(),  "Engineer".to_string(), "Active".to_string(),   "98".to_string()],
        vec!["Bob".to_string(),    "Designer".to_string(), "Active".to_string(),   "87".to_string()],
        vec!["Carol".to_string(),  "Manager".to_string(),  "Away".to_string(),     "76".to_string()],
        vec!["Dave".to_string(),   "Engineer".to_string(), "Inactive".to_string(), "65".to_string()],
        vec!["Eve".to_string(),    "Designer".to_string(), "Active".to_string(),   "91".to_string()],
    ];
    use crate::ui::data_table::data_table_ui::{DataTableColumn, RowAction, BulkAction};
    let columns = vec![
        DataTableColumn::new("name",   "Name",   |r: &Vec<String>| r[0].clone()),
        DataTableColumn::new("role",   "Role",   |r: &Vec<String>| r[1].clone()),
        DataTableColumn::new("status", "Status", |r: &Vec<String>| r[2].clone()),
        DataTableColumn::new("score",  "Score",  |r: &Vec<String>| r[3].clone()),
    ];
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <DataTableBlock
                body=slot!(move || view! {
                    <DataTable
                        data=data.clone()
                        columns=columns.clone()
                        page_size=5
                        show_density=true
                        selectable=true
                        row_actions=vec![
                            RowAction::new("edit",   "Edit").inline(),
                            RowAction::new("delete", "Delete").danger(),
                        ]
                        bulk_actions=vec![
                            BulkAction::new("export", "Export"),
                            BulkAction::new("delete", "Delete").danger(),
                        ]
                    />
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "Full data table with pagination, density toggle, row and bulk actions."
            </p>
        </Stack>
    }
}
