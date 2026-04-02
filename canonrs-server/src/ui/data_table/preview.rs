use leptos::prelude::*;
use super::data_table_ui::{DataTableFull, DataTableColumn};

#[derive(Clone)]
struct User {
    name: String,
    role: String,
    status: String,
    score: u32,
}

#[component]
pub fn DataTableFullShowcasePreview() -> impl IntoView {
    let data: Vec<User> = vec![
        User { name: "Alice".to_string(), role: "Engineer".to_string(), status: "Active".to_string(), score: 98 },
        User { name: "Bob".to_string(), role: "Designer".to_string(), status: "Active".to_string(), score: 87 },
        User { name: "Carol".to_string(), role: "Manager".to_string(), status: "Away".to_string(), score: 76 },
        User { name: "Dave".to_string(), role: "Engineer".to_string(), status: "Inactive".to_string(), score: 65 },
        User { name: "Eve".to_string(), role: "Designer".to_string(), status: "Active".to_string(), score: 91 },
    ];

    let columns: Vec<DataTableColumn<User>> = vec![
        DataTableColumn::new("name", "Name", |u: &User| u.name.clone()),
        DataTableColumn::new("role", "Role", |u: &User| u.role.clone()),
        DataTableColumn::new("status", "Status", |u: &User| u.status.clone()),
        DataTableColumn::new("score", "Score", |u: &User| u.score.to_string()),
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DataTableFull
                    data=data
                    columns=columns
                    page_size=3
                    selectable=true
                    show_density=true
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Table structure and sorting semantics enforced at component level."
            </p>
        </div>
    }
}
