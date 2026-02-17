use leptos::prelude::*;
use super::{List, ListItem, ListItemTitle, ListItemDescription};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <h4>"Basic List"</h4>
                <List>
                    <ListItem>
                        <ListItemTitle>"Item 1"</ListItemTitle>
                        <ListItemDescription>"Description"</ListItemDescription>
                    </ListItem>
                    <ListItem>
                        <ListItemTitle>"Item 2"</ListItemTitle>
                    </ListItem>
                </List>
            </div>

            <div>
                <h4>"Selectable"</h4>
                <List>
                    <ListItem selectable=true selected=true>
                        <ListItemTitle>"Selected"</ListItemTitle>
                    </ListItem>
                    <ListItem selectable=true>
                        <ListItemTitle>"Not selected"</ListItemTitle>
                    </ListItem>
                </List>
            </div>
        </div>
    }
}
