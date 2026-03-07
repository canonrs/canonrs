use leptos::prelude::*;
use std::sync::Arc;
use super::{DataTableFull, DataTableColumn};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
}

#[component]
pub fn BasicExample() -> impl IntoView {
    let users = vec![
        User { id: 1, name: "Alice Johnson".into(), email: "alice@example.com".into(), role: "Admin".into() },
        User { id: 2, name: "Bob Smith".into(), email: "bob@example.com".into(), role: "User".into() },
        User { id: 3, name: "Carol Williams".into(), email: "carol@example.com".into(), role: "Editor".into() },
        User { id: 4, name: "Dave Brown".into(), email: "dave@example.com".into(), role: "Admin".into() },
        User { id: 5, name: "Eve Davis".into(), email: "eve@example.com".into(), role: "User".into() },
        User { id: 6, name: "Frank Miller".into(), email: "frank@example.com".into(), role: "Viewer".into() },
        User { id: 7, name: "Grace Lee".into(), email: "grace@example.com".into(), role: "Editor".into() },
        User { id: 8, name: "Henry Wilson".into(), email: "henry@example.com".into(), role: "Admin".into() },
        User { id: 9, name: "Iris Taylor".into(), email: "iris@example.com".into(), role: "User".into() },
        User { id: 10, name: "Jack Anderson".into(), email: "jack@example.com".into(), role: "Editor".into() },
        User { id: 11, name: "Karen Thomas".into(), email: "karen@example.com".into(), role: "Viewer".into() },
        User { id: 12, name: "Leo Jackson".into(), email: "leo@example.com".into(), role: "Admin".into() },
        User { id: 13, name: "Mia White".into(), email: "mia@example.com".into(), role: "User".into() },
        User { id: 14, name: "Noah Harris".into(), email: "noah@example.com".into(), role: "Editor".into() },
        User { id: 15, name: "Olivia Martin".into(), email: "olivia@example.com".into(), role: "Admin".into() },
    ];

    let columns = vec![
        DataTableColumn::new("id", "ID", |u: &User| u.id.to_string()),
        DataTableColumn::new("name", "Name", |u: &User| u.name.clone()),
        DataTableColumn::new("email", "Email", |u: &User| u.email.clone()),
        DataTableColumn::new("role", "Role", |u: &User| u.role.clone()),
    ];

    view! {
        <DataTableFull 
            data=users 
            columns=columns 
            id="users-table".to_string()
            page_size=5
            selectable=true
            show_density=true
            expand_render=Arc::new(|u: &User| format!("ID: {} | Full info: {} - {} ({})", u.id, u.name, u.email, u.role))
        />
    }
}
