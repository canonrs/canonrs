use leptos::prelude::*;
use super::data_table_island::{DataTableIsland, DataTableIslandColumn};

#[component]
pub fn DataTableStaticShowcasePreview() -> impl IntoView {
    let island_columns = vec![
        DataTableIslandColumn { key: "name".to_string(),   label: "Name".to_string() },
        DataTableIslandColumn { key: "role".to_string(),   label: "Role".to_string() },
        DataTableIslandColumn { key: "status".to_string(), label: "Status".to_string() },
        DataTableIslandColumn { key: "score".to_string(),  label: "Score".to_string() },
    ];

    let island_rows = vec![
        vec!["Alice".to_string(),  "Engineer".to_string(), "Active".to_string(),   "98".to_string()],
        vec!["Bob".to_string(),    "Designer".to_string(), "Active".to_string(),   "87".to_string()],
        vec!["Carol".to_string(),  "Manager".to_string(),  "Away".to_string(),     "76".to_string()],
        vec!["Dave".to_string(),   "Engineer".to_string(), "Inactive".to_string(), "65".to_string()],
        vec!["Eve".to_string(),    "Designer".to_string(), "Active".to_string(),   "91".to_string()],
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DataTableIsland
                    columns=island_columns
                    rows=island_rows
                    page_size=3
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Table structure and sorting semantics enforced at component level."
            </p>
        </div>
    }
}
