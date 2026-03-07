use leptos::prelude::*;
use super::{CommandInteractive, CommandItemData};

#[component]
pub fn BasicExample() -> impl IntoView {
    let items = vec![
        CommandItemData {
            id: "dashboard".to_string(),
            label: "Dashboard".to_string(),
            value: "/dashboard".to_string(),
            keywords: vec!["home".to_string(), "overview".to_string()],
            group: Some("Navigation".to_string()),
        },
        CommandItemData {
            id: "projects".to_string(),
            label: "Projects".to_string(),
            value: "/projects".to_string(),
            keywords: vec!["work".to_string(), "portfolio".to_string()],
            group: Some("Navigation".to_string()),
        },
        CommandItemData {
            id: "tasks".to_string(),
            label: "Tasks".to_string(),
            value: "/tasks".to_string(),
            keywords: vec!["todo".to_string(), "work".to_string()],
            group: Some("Navigation".to_string()),
        },
        CommandItemData {
            id: "profile".to_string(),
            label: "Profile".to_string(),
            value: "/profile".to_string(),
            keywords: vec!["user".to_string(), "account".to_string()],
            group: Some("Settings".to_string()),
        },
    ];
    
    view! {
        <div style="width: 400px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md);">
            <CommandInteractive
                items=items
                placeholder="Search commands...".to_string()
            />
        </div>
    }
}
