use leptos::prelude::*;
use super::tabs_island::{TabsIsland, TabItem};

#[component]
pub fn TabsShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TabsIsland
                    tabs=vec![
                        TabItem { value: "overview".into(), label: "Overview".into(), content: "Overview content — structure drives state.".into(), disabled: false },
                        TabItem { value: "api".into(),      label: "API".into(),      content: "API reference content.".into(),                   disabled: false },
                        TabItem { value: "examples".into(), label: "Examples".into(), content: "Examples content.".into(),                        disabled: false },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tab selection governed by signal — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Second tab active"</span>
                <div data-rs-showcase-preview-row="">
                    <TabsIsland
                        tabs=vec![
                            TabItem { value: "a".into(), label: "Tab A".into(), content: "Content A".into(), disabled: false },
                            TabItem { value: "b".into(), label: "Tab B".into(), content: "Content B".into(), disabled: false },
                            TabItem { value: "c".into(), label: "Tab C".into(), content: "Content C".into(), disabled: false },
                        ]
                    />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With disabled tab"</span>
                <div data-rs-showcase-preview-row="">
                    <TabsIsland
                        tabs=vec![
                            TabItem { value: "x".into(), label: "Active".into(),   content: "Active content.".into(),   disabled: false },
                            TabItem { value: "y".into(), label: "Disabled".into(), content: "Disabled content.".into(), disabled: true  },
                            TabItem { value: "z".into(), label: "Normal".into(),   content: "Normal content.".into(),   disabled: false },
                        ]
                    />
                </div>
            </div>
        </div>
    }
}
