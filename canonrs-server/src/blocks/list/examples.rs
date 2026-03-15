use leptos::prelude::*;
use super::{List, ListItem};

pub fn basic_example() -> impl IntoView {
    view! {
        <List>
            <ListItem>"Item 1"</ListItem>
            <ListItem>"Item 2"</ListItem>
            <ListItem selected=true>"Item 3 (selected)"</ListItem>
        </List>
    }
}
