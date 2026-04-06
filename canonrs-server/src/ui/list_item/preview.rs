use leptos::prelude::*;
use super::list_item_island::{ListItemIsland, ListIslandItem};

#[component]
pub fn ListItemShowcasePreview() -> impl IntoView {
    let items = vec![
        ListIslandItem { label: "Alice Johnson".into(), description: Some("Engineer · Active".into()), disabled: false },
        ListIslandItem { label: "Bob Smith".into(),     description: Some("Designer · Away".into()),   disabled: false },
        ListIslandItem { label: "Carol White".into(),   description: Some("Manager · Active".into()),  disabled: false },
    ];

    let items_selectable = vec![
        ListIslandItem { label: "Alice Johnson".into(), description: Some("Engineer".into()), disabled: false },
        ListIslandItem { label: "Bob Smith".into(),     description: Some("Designer".into()), disabled: false },
        ListIslandItem { label: "Carol White".into(),   description: Some("Disabled".into()),  disabled: true  },
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ListItemIsland items=items />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection and interaction states encoded via structured attributes."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Selectable"</span>
                <div data-rs-showcase-preview-row="">
                    <ListItemIsland items=items_selectable multiple=false />
                </div>
            </div>
        </div>
    }
}
