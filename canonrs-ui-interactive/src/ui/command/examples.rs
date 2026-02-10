use leptos::prelude::*;
use super::CommandInteractive;

#[component]
pub fn BasicExample() -> impl IntoView {
    let items: Vec<String> = vec![
        "Copy".to_string(),
        "Cut".to_string(),
        "Paste".to_string(),
        "Delete".to_string(),
        "Rename".to_string(),
    ];
    
    view! {
        <CommandInteractive 
            id="command-example"
            items=items
        />
    }
}
