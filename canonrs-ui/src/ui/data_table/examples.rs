use leptos::prelude::*;
use super::{DataTable, types::DataTableColumn};

#[derive(Clone)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[component]
pub fn basic_example() -> impl IntoView {
    let data = vec![
        User { id: "1".to_string(), name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: "2".to_string(), name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ];

    let columns = vec![
        DataTableColumn {
            id: "name".to_string(),
            label: "Name".to_string(),
            render: Box::new(|user: &User| view! { <span>{user.name.clone()}</span> }.into_any()),
            sortable: true,
            width: None,
        },
        DataTableColumn {
            id: "email".to_string(),
            label: "Email".to_string(),
            render: Box::new(|user: &User| view! { <span>{user.email.clone()}</span> }.into_any()),
            sortable: true,
            width: None,
        },
    ];

    view! {
        <DataTable data=data columns=columns />
    }
}
