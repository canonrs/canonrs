//! DataTable data types — config e mock data

use super::data_table_ui::{DataTableColumn, RowAction, BulkAction};
use canonrs_core::primitives::DataTableDensity;

#[derive(Clone, Debug)]
pub struct UserRow {
    pub id: u32,
    pub name: &'static str,
    pub email: &'static str,
    pub role: &'static str,
    pub status: &'static str,
}

pub fn mock_users() -> Vec<UserRow> {
    vec![
        UserRow { id: 1, name: "Alice Johnson",  email: "alice@example.com",  role: "Admin",  status: "Active"   },
        UserRow { id: 2, name: "Bob Smith",       email: "bob@example.com",    role: "Editor", status: "Active"   },
        UserRow { id: 3, name: "Carol Williams",  email: "carol@example.com",  role: "Viewer", status: "Inactive" },
        UserRow { id: 4, name: "David Brown",     email: "david@example.com",  role: "Editor", status: "Active"   },
        UserRow { id: 5, name: "Eve Davis",       email: "eve@example.com",    role: "Admin",  status: "Active"   },
        UserRow { id: 6, name: "Frank Miller",    email: "frank@example.com",  role: "Viewer", status: "Inactive" },
        UserRow { id: 7, name: "Grace Lee",       email: "grace@example.com",  role: "Editor", status: "Active"   },
        UserRow { id: 8, name: "Henry Wilson",    email: "henry@example.com",  role: "Admin",  status: "Active"   },
    ]
}

pub fn user_columns() -> Vec<DataTableColumn<UserRow>> {
    vec![
        DataTableColumn::new("name",   "Name",   |u: &UserRow| u.name.to_string()),
        DataTableColumn::new("email",  "Email",  |u: &UserRow| u.email.to_string()),
        DataTableColumn::new("role",   "Role",   |u: &UserRow| u.role.to_string()),
        DataTableColumn::new("status", "Status", |u: &UserRow| u.status.to_string()),
    ]
}

#[derive(Clone, Debug)]
pub struct DataTableConfig {
    pub page_size:    usize,
    pub selectable:   bool,
    pub show_density: bool,
    pub resizable:    bool,
    pub density:      DataTableDensity,
    pub row_actions:  Vec<RowAction>,
    pub bulk_actions: Vec<BulkAction>,
}

impl Default for DataTableConfig {
    fn default() -> Self {
        Self {
            page_size:    5,
            selectable:   false,
            show_density: false,
            resizable:    false,
            density:      DataTableDensity::default(),
            row_actions:  vec![],
            bulk_actions: vec![],
        }
    }
}
