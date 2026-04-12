use leptos::prelude::*;
use super::list_item_boundary::{List, ListItem, ListItemTitle, ListItemDescription};

#[component]
pub fn ListItemShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <List>
                    <ListItem>
                        <ListItemTitle>"Alice Johnson"</ListItemTitle>
                        <ListItemDescription>"Engineer · Active"</ListItemDescription>
                    </ListItem>
                    <ListItem>
                        <ListItemTitle>"Bob Smith"</ListItemTitle>
                        <ListItemDescription>"Designer · Away"</ListItemDescription>
                    </ListItem>
                    <ListItem>
                        <ListItemTitle>"Carol White"</ListItemTitle>
                        <ListItemDescription>"Manager · Active"</ListItemDescription>
                    </ListItem>
                </List>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection and interaction states encoded via structured attributes."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Selectable"</span>
                <div data-rs-showcase-preview-row="">
                    <List>
                        <ListItem selectable=true>
                            <ListItemTitle>"Alice Johnson"</ListItemTitle>
                            <ListItemDescription>"Engineer"</ListItemDescription>
                        </ListItem>
                        <ListItem selectable=true>
                            <ListItemTitle>"Bob Smith"</ListItemTitle>
                            <ListItemDescription>"Designer"</ListItemDescription>
                        </ListItem>
                        <ListItem selectable=true disabled=true>
                            <ListItemTitle>"Carol White"</ListItemTitle>
                            <ListItemDescription>"Disabled"</ListItemDescription>
                        </ListItem>
                    </List>
                </div>
            </div>
        </div>
    }
}
