use leptos::prelude::*;
use super::select_island::{SelectIsland, SelectOption};
use super::select_ui::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem};
use canonrs_core::meta::DisabledState;

#[component]
pub fn SelectShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SelectIsland
                    placeholder="Choose a framework..."
                    options=vec![
                        SelectOption { value: "leptos".into(),  label: "Leptos".into(),         disabled: false },
                        SelectOption { value: "dioxus".into(),  label: "Dioxus".into(),         disabled: false },
                        SelectOption { value: "yew".into(),     label: "Yew (disabled)".into(), disabled: true  },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection, visibility and interaction fully governed by structure."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    // SSR: estado inicial via SelectItem selected=Selected
                    <Select>
                        <SelectTrigger>
                            <SelectValue placeholder="Select size...">"Medium"</SelectValue>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="sm">"Small"</SelectItem>
                            <SelectItem value="md" selected=canonrs_core::meta::SelectionState::Selected>"Medium"</SelectItem>
                            <SelectItem value="lg">"Large"</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With disabled options"</span>
                <div data-rs-showcase-preview-row="">
                    <SelectIsland
                        placeholder="Select plan..."
                        options=vec![
                            SelectOption { value: "free".into(),       label: "Free".into(),                  disabled: false },
                            SelectOption { value: "pro".into(),        label: "Pro".into(),                   disabled: false },
                            SelectOption { value: "enterprise".into(), label: "Enterprise (disabled)".into(), disabled: true  },
                            SelectOption { value: "custom".into(),     label: "Custom (disabled)".into(),     disabled: true  },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Many options — keyboard nav"</span>
                <div data-rs-showcase-preview-row="">
                    <SelectIsland
                        placeholder="Select country..."
                        options=vec![
                            SelectOption { value: "br".into(), label: "Brazil".into(),         disabled: false },
                            SelectOption { value: "us".into(), label: "United States".into(),  disabled: false },
                            SelectOption { value: "de".into(), label: "Germany".into(),        disabled: false },
                            SelectOption { value: "jp".into(), label: "Japan".into(),          disabled: false },
                            SelectOption { value: "fr".into(), label: "France".into(),         disabled: false },
                            SelectOption { value: "uk".into(), label: "United Kingdom".into(), disabled: false },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <Select disabled=DisabledState::Disabled>
                        <SelectTrigger disabled=DisabledState::Disabled>
                            <SelectValue placeholder="Disabled...">{""}</SelectValue>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="a">"Option A"</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </div>

        </div>
    }
}
