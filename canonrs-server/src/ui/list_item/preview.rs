use leptos::prelude::*;
use super::list_item_island::{ListIsland, ListItemIsland, ListItemTitleIsland, ListItemDescriptionIsland};

#[component]
pub fn ListItemShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ListIsland>
                    <ListItemIsland>
                        <ListItemTitleIsland>"Alice Johnson"</ListItemTitleIsland>
                        <ListItemDescriptionIsland>"Engineer · Active"</ListItemDescriptionIsland>
                    </ListItemIsland>
                    <ListItemIsland>
                        <ListItemTitleIsland>"Bob Smith"</ListItemTitleIsland>
                        <ListItemDescriptionIsland>"Designer · Away"</ListItemDescriptionIsland>
                    </ListItemIsland>
                    <ListItemIsland>
                        <ListItemTitleIsland>"Carol White"</ListItemTitleIsland>
                        <ListItemDescriptionIsland>"Manager · Active"</ListItemDescriptionIsland>
                    </ListItemIsland>
                </ListIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection and interaction states encoded via structured attributes."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Selectable"</span>
                <div data-rs-showcase-preview-row="">
                    <ListIsland>
                        <ListItemIsland selectable=true>
                            <ListItemTitleIsland>"Alice Johnson"</ListItemTitleIsland>
                            <ListItemDescriptionIsland>"Engineer"</ListItemDescriptionIsland>
                        </ListItemIsland>
                        <ListItemIsland selectable=true>
                            <ListItemTitleIsland>"Bob Smith"</ListItemTitleIsland>
                            <ListItemDescriptionIsland>"Designer"</ListItemDescriptionIsland>
                        </ListItemIsland>
                        <ListItemIsland selectable=true disabled=true>
                            <ListItemTitleIsland>"Carol White"</ListItemTitleIsland>
                            <ListItemDescriptionIsland>"Disabled"</ListItemDescriptionIsland>
                        </ListItemIsland>
                    </ListIsland>
                </div>
            </div>
        </div>
    }
}
