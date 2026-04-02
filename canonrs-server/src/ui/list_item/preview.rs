use leptos::prelude::*;
use super::list_item_ui::{List, ListItem, ListItemTitle, ListItemDescription, ListSelectionMode};

#[component]
pub fn ListItemShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <List>
                    <ListItem>
                        <ListItemTitle><span>{"Alice Johnson"}</span></ListItemTitle>
                        <ListItemDescription><span>{"Engineer · Active"}</span></ListItemDescription>
                    </ListItem>
                    <ListItem>
                        <ListItemTitle><span>{"Bob Smith"}</span></ListItemTitle>
                        <ListItemDescription><span>{"Designer · Away"}</span></ListItemDescription>
                    </ListItem>
                    <ListItem>
                        <ListItemTitle><span>{"Carol White"}</span></ListItemTitle>
                        <ListItemDescription><span>{"Manager · Active"}</span></ListItemDescription>
                    </ListItem>
                </List>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection and interaction states encoded via structured attributes."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Selectable"</span>
                <div data-rs-showcase-preview-row="">
                    <List _selection_mode=ListSelectionMode::Single>
                        <ListItem selectable=true selected=true>
                            <ListItemTitle><span>{"Alice Johnson"}</span></ListItemTitle>
                            <ListItemDescription><span>{"Engineer"}</span></ListItemDescription>
                        </ListItem>
                        <ListItem selectable=true>
                            <ListItemTitle><span>{"Bob Smith"}</span></ListItemTitle>
                            <ListItemDescription><span>{"Designer"}</span></ListItemDescription>
                        </ListItem>
                        <ListItem selectable=true disabled=true>
                            <ListItemTitle><span>{"Carol White"}</span></ListItemTitle>
                            <ListItemDescription><span>{"Disabled"}</span></ListItemDescription>
                        </ListItem>
                    </List>
                </div>
            </div>
        </div>
    }
}
