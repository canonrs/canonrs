use leptos::prelude::*;
use super::data_table_boundary::DataTable;
use crate::ui::data_table::data_table_ui::{DataTableColumn, RowAction, BulkAction};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols, GridGap};

fn sample_data() -> Vec<Vec<String>> {
    vec![
        vec!["Alice".to_string(),   "Engineer".to_string(), "Active".to_string(),   "98".to_string()],
        vec!["Bob".to_string(),     "Designer".to_string(), "Active".to_string(),   "87".to_string()],
        vec!["Carol".to_string(),   "Manager".to_string(),  "Away".to_string(),     "76".to_string()],
        vec!["Dave".to_string(),    "Engineer".to_string(), "Inactive".to_string(), "65".to_string()],
        vec!["Eve".to_string(),     "Designer".to_string(), "Active".to_string(),   "91".to_string()],
        vec!["Frank".to_string(),   "DevOps".to_string(),   "Active".to_string(),   "82".to_string()],
        vec!["Grace".to_string(),   "QA".to_string(),       "Active".to_string(),   "79".to_string()],
        vec!["Henry".to_string(),   "Manager".to_string(),  "Away".to_string(),     "88".to_string()],
        vec!["Iris".to_string(),    "Engineer".to_string(), "Active".to_string(),   "95".to_string()],
        vec!["Jack".to_string(),    "Designer".to_string(), "Inactive".to_string(), "71".to_string()],
        vec!["Karen".to_string(),   "DevOps".to_string(),   "Active".to_string(),   "84".to_string()],
        vec!["Leo".to_string(),     "QA".to_string(),       "Active".to_string(),   "77".to_string()],
    ]
}

fn sample_columns() -> Vec<DataTableColumn<Vec<String>>> {
    vec![
        DataTableColumn::new("name",   "Name",   |r: &Vec<String>| r[0].clone()),
        DataTableColumn::new("role",   "Role",   |r: &Vec<String>| r[1].clone()),
        DataTableColumn::new("status", "Status", |r: &Vec<String>| r[2].clone()),
        DataTableColumn::new("score",  "Score",  |r: &Vec<String>| r[3].clone()),
    ]
}

fn expand_data() -> Vec<Vec<String>> {
    vec![
        vec!["Alice".to_string(),   "Engineer".to_string(), "Active".to_string(),   "alice@example.com|alice.j@work.com".to_string()],
        vec!["Bob".to_string(),     "Designer".to_string(), "Active".to_string(),   "bob@example.com|bob.s@work.com".to_string()],
        vec!["Carol".to_string(),   "Manager".to_string(),  "Away".to_string(),     "carol@example.com|carol.w@work.com".to_string()],
        vec!["Dave".to_string(),    "Engineer".to_string(), "Inactive".to_string(), "dave@example.com|dave.b@work.com".to_string()],
        vec!["Eve".to_string(),     "Designer".to_string(), "Active".to_string(),   "eve@example.com|eve.d@work.com".to_string()],
        vec!["Frank".to_string(),   "DevOps".to_string(),   "Active".to_string(),   "frank@example.com|frank.m@work.com".to_string()],
    ]
}

fn expand_columns() -> Vec<DataTableColumn<Vec<String>>> {
    vec![
        DataTableColumn::new("name",   "Name",   |r: &Vec<String>| r[0].clone()),
        DataTableColumn::new("role",   "Role",   |r: &Vec<String>| r[1].clone()),
        DataTableColumn::new("status", "Status", |r: &Vec<String>| r[2].clone()),
        DataTableColumn::new("email",  "Email",  |r: &Vec<String>| r[3].split('|').next().unwrap_or("").to_string()),
    ]
}

#[component]
pub fn DataTableStaticShowcasePreview() -> impl IntoView {
    view! {
        <DataTable
            data=sample_data()
            columns=sample_columns()
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
    }
}

#[component]
pub fn DataTableShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Xl>
            <Grid cols=GridCols::Two gap=GridGap::Lg>

                // 1. Data Operations — sort, filter, pagination
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Data Operations — Sort · Search · Paginate"</span>
                    <DataTable
                        data=sample_data()
                        columns=sample_columns()
                        page_size=5
                    />
                </Stack>

                // 2. Visual Density
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Visual Density — Compact · Comfortable · Spacious"</span>
                    <DataTable
                        data=sample_data()
                        columns=sample_columns()
                        page_size=5
                        show_density=true
                    />
                </Stack>

                // 3. Selection Domain — row selection, bulk actions, keyboard nav
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Selection — Row · Bulk · Keyboard"</span>
                    <DataTable
                        data=sample_data()
                        columns=sample_columns()
                        page_size=5
                        selectable=true
                        bulk_actions=vec![
                            BulkAction::new("export", "Export"),
                            BulkAction::new("delete", "Delete").danger(),
                        ]
                    />
                </Stack>

                // 4. Action Dispatch — inline actions, menu actions, context menu
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Actions — Inline · Menu · Context"</span>
                    <DataTable
                        data=sample_data()
                        columns=sample_columns()
                        page_size=5
                        row_actions=vec![
                            RowAction::new("edit",   "Edit").inline(),
                            RowAction::new("delete", "Delete").danger(),
                            RowAction::new("archive", "Archive"),
                        ]
                    />
                </Stack>

                // 5. Expandable Structure
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Expand Row — Structural Composition"</span>
                    <DataTable
                        data=expand_data()
                        columns=expand_columns()
                        page_size=5
                        expand_render=std::sync::Arc::new(|r: &Vec<String>| {
                            let emails: Vec<String> = r[3].split('|').map(|s| s.to_string()).collect();
                            view! {
                                <div style="padding:var(--space-sm);display:flex;flex-direction:column;gap:var(--space-xs);">
                                    <strong>"Contact emails:"</strong>
                                    {emails.into_iter().map(|e| view! {
                                        <span style="color:var(--theme-action-primary-bg);">{e}</span>
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        })
                    />
                </Stack>

                // 6. Column Layout — resize, reorder, pin, toggle
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Column Layout — Resize · Reorder · Toggle"</span>
                    <DataTable
                        data=sample_data()
                        columns=sample_columns()
                        page_size=5
                        resizable=true
                    />
                </Stack>

                // 7. Editing Domain — inline edit lifecycle: state transition, commit, rollback
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Editing — Inline Edit · Commit · Rollback"</span>
                    <DataTable
                        data=sample_data()
                        columns=sample_columns()
                        page_size=5
                        row_actions=vec![
                            RowAction::new("edit",   "✎ Edit").inline(),
                        ]
                    />
                </Stack>

                // 8. Full Orchestration — all capabilities composed
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-label="">"Full Orchestration"</span>
                    <DataTable
                        data=expand_data()
                        columns=expand_columns()
                        page_size=3
                        show_density=true
                        selectable=true
                        resizable=true
                        row_actions=vec![
                            RowAction::new("edit",   "Edit").inline(),
                            RowAction::new("delete", "Delete").danger(),
                            RowAction::new("archive", "Archive"),
                        ]
                        bulk_actions=vec![
                            BulkAction::new("export", "Export"),
                            BulkAction::new("delete", "Delete").danger(),
                        ]
                        expand_render=std::sync::Arc::new(|r: &Vec<String>| {
                            let emails: Vec<String> = r[3].split('|').map(|s| s.to_string()).collect();
                            view! {
                                <div style="padding:var(--space-sm);display:flex;flex-direction:column;gap:var(--space-xs);">
                                    <strong>"Contact emails:"</strong>
                                    {emails.into_iter().map(|e| view! {
                                        <span style="color:var(--theme-action-primary-bg);">{e}</span>
                                    }).collect::<Vec<_>>()}
                                </div>
                            }.into_any()
                        })
                    />
                </Stack>

            </Grid>
            <p data-rs-showcase-preview-anchor="">
                "DataTable — Data Ops · Density · Selection · Actions · Expand · Column Layout · Editing · Full Orchestration"
            </p>
        </Stack>
    }
}
