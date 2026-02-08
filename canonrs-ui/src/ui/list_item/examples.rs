use leptos::prelude::*;
use super::list_item_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <ListItem>"Item 1"</ListItem>
            <ListItem>"Item 2"</ListItem>
            <ListItem>"Item 3"</ListItem>
        </div>
    }
}
