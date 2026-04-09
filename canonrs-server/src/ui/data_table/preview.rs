use leptos::prelude::*;
use super::data_table_ui::DataTableStatic;
use super::data_table_ui::DataTableColumn;

#[component]
pub fn DataTableStaticShowcasePreview() -> impl IntoView {
    let columns: Vec<DataTableColumn<Vec<String>>> = vec![
        DataTableColumn::new("name",   "Name",   |r: &Vec<String>| r[0].clone()),
        DataTableColumn::new("role",   "Role",   |r: &Vec<String>| r[1].clone()),
        DataTableColumn::new("status", "Status", |r: &Vec<String>| r[2].clone()),
        DataTableColumn::new("score",  "Score",  |r: &Vec<String>| r[3].clone()),
    ];

    let data = vec![
        vec!["Alice".to_string(),  "Engineer".to_string(), "Active".to_string(),   "98".to_string()],
        vec!["Bob".to_string(),    "Designer".to_string(), "Active".to_string(),   "87".to_string()],
        vec!["Carol".to_string(),  "Manager".to_string(),  "Away".to_string(),     "76".to_string()],
        vec!["Dave".to_string(),   "Engineer".to_string(), "Inactive".to_string(), "65".to_string()],
        vec!["Eve".to_string(),    "Designer".to_string(), "Active".to_string(),   "91".to_string()],
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DataTableStatic
                    data=data
                    columns=columns
                    page_size=3
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Table structure and sorting semantics enforced at component level."
            </p>
        </div>
    }
}
