use leptos::prelude::*;
use rs_design::ui::datagrid::{DataGrid, ColumnDef, DataGridConfig, SelectionMode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub last_login: String,
}

#[component]
pub fn DataGridTab() -> impl IntoView {
    // Sample data (generate 1000 rows for virtual scroll demo)
    let users = (1..=1000).map(|i| User {
        id: i,
        name: format!("User {}", i),
        email: format!("user{}@example.com", i),
        role: match i % 3 {
            0 => "Admin".to_string(),
            1 => "Editor".to_string(),
            _ => "Viewer".to_string(),
        },
        status: if i % 5 == 0 { "Inactive".to_string() } else { "Active".to_string() },
        last_login: format!("2025-01-{:02}", (i % 28) + 1),
    }).collect::<Vec<_>>();
    
    let (data, set_data) = signal(users);
    
    // Define columns
    let columns = vec![
        ColumnDef::new("id", "ID", |user: &User| user.id.to_string())
            .with_width("80px")
            .sortable(),
        
        ColumnDef::new("name", "Name", |user: &User| user.name.clone())
            .with_width("200px")
            .sortable()
            .filterable(),
        
        ColumnDef::new("email", "Email", |user: &User| user.email.clone())
            .with_width("250px")
            .sortable()
            .filterable(),
        
        ColumnDef::new("role", "Role", |user: &User| user.role.clone())
            .with_width("120px")
            .sortable()
            .filterable()
            .with_cell_renderer(|user: &User| {
                let badge_class = match user.role.as_str() {
                    "Admin" => "bg-red-100 text-red-800",
                    "Editor" => "bg-blue-100 text-blue-800",
                    _ => "bg-gray-100 text-gray-800",
                };
                
                view! {
                    <span class=format!("px-2 py-1 rounded text-xs {}", badge_class)>
                        {user.role.clone()}
                    </span>
                }.into_any()
            }),
        
        ColumnDef::new("status", "Status", |user: &User| user.status.clone())
            .with_width("120px")
            .sortable()
            .filterable()
            .with_cell_renderer(|user: &User| {
                let (icon, color) = if user.status == "Active" {
                    ("●", "text-green-600")
                } else {
                    ("●", "text-gray-400")
                };
                
                view! {
                    <div class="flex items-center gap-2">
                        <span class=color>{icon}</span>
                        <span>{user.status.clone()}</span>
                    </div>
                }.into_any()
            }),
        
        ColumnDef::new("last_login", "Last Login", |user: &User| user.last_login.clone())
            .with_width("150px")
            .sortable(),
    ];
    
    // Config for virtual scroll
    let config_virtual = DataGridConfig::new()
        .with_sorting(true)
        .with_filtering(true)
        .with_selection(SelectionMode::Multiple)
        .striped();
    
    // Config for pagination
    let config_paginated = DataGridConfig::new()
        .with_sorting(true)
        .with_filtering(true)
        .with_pagination(20)
        .with_selection(SelectionMode::Single);
    
    let (mode, set_mode) = signal("virtual");
    
    let on_row_click = Callback::new(move |user: User| {
        leptos::logging::log!("🖱️ Clicked row: {:?}", user);
    });
    
    let on_selection_change = Callback::new(move |selected: Vec<usize>| {
        leptos::logging::log!("✅ Selection changed: {:?}", selected);
    });
    
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-2xl font-bold mb-2">"DataGrid"</h2>
                    <p class="text-muted-foreground">"Enterprise data table with virtual scroll, sorting, and filtering"</p>
                </div>
                
                <div class="flex gap-2">
                    <button
                        class=move || {
                            let base = "px-4 py-2 text-sm rounded";
                            if mode.get() == "virtual" {
                                format!("{} bg-primary text-primary-foreground", base)
                            } else {
                                format!("{} border hover:bg-muted", base)
                            }
                        }
                        on:click=move |_| set_mode.set("virtual")
                    >
                        "Virtual Scroll"
                    </button>
                    
                    <button
                        class=move || {
                            let base = "px-4 py-2 text-sm rounded";
                            if mode.get() == "paginated" {
                                format!("{} bg-primary text-primary-foreground", base)
                            } else {
                                format!("{} border hover:bg-muted", base)
                            }
                        }
                        on:click=move |_| set_mode.set("paginated")
                    >
                        "Pagination"
                    </button>
                </div>
            </div>
            
            {move || {
                if mode.get() == "virtual" {
                    view! {
                        <div>
                            <DataGrid
                                data=data
                                columns=columns.clone()
                                config=config_virtual.clone()
                                on_row_click=on_row_click
                                on_selection_change=on_selection_change
                            />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div>
                            <DataGrid
                                data=data
                                columns=columns.clone()
                                config=config_paginated.clone()
                                on_row_click=on_row_click
                                on_selection_change=on_selection_change
                            />
                        </div>
                    }.into_any()
                }
            }}
            
            <div class="grid grid-cols-2 gap-4">
                <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                    <p class="text-sm font-semibold text-blue-900 mb-2">"Virtual Scroll Mode"</p>
                    <ul class="text-xs text-blue-700 space-y-1">
                        <li>"✅ 1000 rows rendered smoothly"</li>
                        <li>"✅ Only visible rows in DOM"</li>
                        <li>"✅ Multi-column sorting (Shift+Click)"</li>
                        <li>"✅ Column filtering"</li>
                        <li>"✅ Multiple selection"</li>
                    </ul>
                </div>
                
                <div class="p-4 bg-green-50 border border-green-200 rounded">
                    <p class="text-sm font-semibold text-green-900 mb-2">"Pagination Mode"</p>
                    <ul class="text-xs text-green-700 space-y-1">
                        <li>"✅ 20 rows per page"</li>
                        <li>"✅ Previous/Next navigation"</li>
                        <li>"✅ Sorting + filtering"</li>
                        <li>"✅ Single selection"</li>
                        <li>"✅ Row count display"</li>
                    </ul>
                </div>
            </div>
            
            <div class="p-4 bg-purple-50 border border-purple-200 rounded">
                <p class="text-sm font-semibold text-purple-900 mb-2">"Features Demonstrated"</p>
                <div class="grid grid-cols-3 gap-4 mt-3">
                    <div>
                        <p class="text-xs font-semibold text-purple-800">"Sorting"</p>
                        <p class="text-xs text-purple-600">"Click header to sort"</p>
                        <p class="text-xs text-purple-600">"Shift+Click for multi-column"</p>
                    </div>
                    <div>
                        <p class="text-xs font-semibold text-purple-800">"Filtering"</p>
                        <p class="text-xs text-purple-600">"Type in column filters"</p>
                        <p class="text-xs text-purple-600">"Case-insensitive search"</p>
                    </div>
                    <div>
                        <p class="text-xs font-semibold text-purple-800">"Selection"</p>
                        <p class="text-xs text-purple-600">"Click rows to select"</p>
                        <p class="text-xs text-purple-600">"Check console for events"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
