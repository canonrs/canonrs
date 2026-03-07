use leptos::prelude::*;
use super::{List, ListItem, ListItemTitle, ListItemDescription};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 3rem;">
            // Basic static list
            <div>
                <h4>"Static List (no interaction)"</h4>
                <List>
                    <ListItem>
                        <ListItemTitle>"Item 1"</ListItemTitle>
                        <ListItemDescription>"Read-only item"</ListItemDescription>
                    </ListItem>
                    <ListItem>
                        <ListItemTitle>"Item 2"</ListItemTitle>
                        <ListItemDescription>"Another read-only item"</ListItemDescription>
                    </ListItem>
                </List>
            </div>

            // Selectable list (behavior attached)
            <div>
                <h4>"Selectable List (click or press Enter/Space)"</h4>
                <List id="list-1">
                    <ListItem id="item-1" selectable=true>
                        <ListItemTitle>"Option 1"</ListItemTitle>
                        <ListItemDescription>"Click to select"</ListItemDescription>
                    </ListItem>
                    <ListItem id="item-2" selectable=true selected=true>
                        <ListItemTitle>"Option 2"</ListItemTitle>
                        <ListItemDescription>"Initially selected"</ListItemDescription>
                    </ListItem>
                    <ListItem id="item-3" selectable=true>
                        <ListItemTitle>"Option 3"</ListItemTitle>
                        <ListItemDescription>"Another option"</ListItemDescription>
                    </ListItem>
                    <ListItem id="item-4" selectable=true disabled=true>
                        <ListItemTitle>"Option 4 (Disabled)"</ListItemTitle>
                        <ListItemDescription>"Cannot be selected"</ListItemDescription>
                    </ListItem>
                </List>
            </div>

            // Settings menu example
            <div>
                <h4>"Settings Menu"</h4>
                <List id="settings-list">
                    <ListItem id="notif" selectable=true selected=true>
                        <ListItemTitle>"🔔 Notifications"</ListItemTitle>
                        <ListItemDescription>"Email, push, and in-app alerts"</ListItemDescription>
                    </ListItem>
                    <ListItem id="privacy" selectable=true>
                        <ListItemTitle>"🔐 Privacy & Security"</ListItemTitle>
                        <ListItemDescription>"Data protection and permissions"</ListItemDescription>
                    </ListItem>
                    <ListItem id="billing" selectable=true>
                        <ListItemTitle>"💳 Billing"</ListItemTitle>
                        <ListItemDescription>"Plans and payment methods"</ListItemDescription>
                    </ListItem>
                </List>
            </div>
        </div>
    }
}
