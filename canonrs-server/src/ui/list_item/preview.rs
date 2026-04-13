use leptos::prelude::*;
use super::list_item_boundary::{List, ListItem, ListItemTitle, ListItemDescription};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ListItemShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
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
            <p data-rs-showcase-preview-anchor="">
                "Selection and interaction states encoded via structured attributes."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Selectable"</span>
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
            </Stack>
        </Stack>
    }
}
