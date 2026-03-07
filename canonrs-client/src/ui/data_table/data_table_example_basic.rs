use leptos::prelude::*;
use super::{DataTableInteractive, DataTableRequest, DataTableResponse, ColumnDef};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

#[component]
pub fn DataTableInteractiveExample() -> impl IntoView {
    let fetch_users = move |req: DataTableRequest| -> Result<DataTableResponse<User>, String> {
        let mut all_users = vec![
            User { id: 1, name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string() },
            User { id: 2, name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), role: "Editor".to_string(), status: "Active".to_string() },
            User { id: 3, name: "Carol Williams".to_string(), email: "carol@example.com".to_string(), role: "Viewer".to_string(), status: "Inactive".to_string() },
            User { id: 4, name: "David Brown".to_string(), email: "david@example.com".to_string(), role: "Editor".to_string(), status: "Active".to_string() },
            User { id: 5, name: "Eve Davis".to_string(), email: "eve@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string() },
            User { id: 6, name: "Frank Miller".to_string(), email: "frank@example.com".to_string(), role: "Viewer".to_string(), status: "Inactive".to_string() },
            User { id: 7, name: "Grace Lee".to_string(), email: "grace@example.com".to_string(), role: "Editor".to_string(), status: "Active".to_string() },
            User { id: 8, name: "Henry Wilson".to_string(), email: "henry@example.com".to_string(), role: "Admin".to_string(), status: "Active".to_string() },
        ];

        if !req.filter_query.is_empty() {
            let query = req.filter_query.to_lowercase();
            all_users.retain(|user| {
                user.name.to_lowercase().contains(&query) ||
                user.email.to_lowercase().contains(&query) ||
                user.role.to_lowercase().contains(&query) ||
                user.status.to_lowercase().contains(&query)
            });
        }

        if let Some(ref col) = req.sort_column {
            all_users.sort_by(|a, b| {
                let cmp = match col.as_str() {
                    "name"   => a.name.cmp(&b.name),
                    "email"  => a.email.cmp(&b.email),
                    "role"   => a.role.cmp(&b.role),
                    "status" => a.status.cmp(&b.status),
                    _ => std::cmp::Ordering::Equal,
                };
                if req.sort_ascending { cmp } else { cmp.reverse() }
            });
        }

        let total = all_users.len();
        let start = (req.page - 1) * req.page_size;
        let end = (start + req.page_size).min(total);
        let data = all_users.into_iter().skip(start).take(end - start).collect();

        Ok(DataTableResponse { data, total, page: req.page, page_size: req.page_size })
    };

    let columns = Signal::derive(move || vec![
        ColumnDef::new("name", "Name", |user: &User| user.name.clone()),
        ColumnDef::new("email", "Email", |user: &User| user.email.clone()),
        ColumnDef::new("role", "Role", |user: &User| user.role.clone()),
        ColumnDef::new("status", "Status", |user: &User| user.status.clone()),
    ]);

    view! {
        <DataTableInteractive
            columns=columns
            fetch_data=fetch_users
            page_size=5
            id="users-table"
        />
    }
}
