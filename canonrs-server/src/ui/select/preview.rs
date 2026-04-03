use leptos::prelude::*;
use super::select_ui::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem, SelectSeparator};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn SelectShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Select>
                    <SelectTrigger>
                        <SelectValue placeholder="Choose a framework...">"All Frameworks"</SelectValue>
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="leptos">"Leptos"</SelectItem>
                        <SelectItem value="dioxus">"Dioxus"</SelectItem>
                        <SelectSeparator/>
                        <SelectItem value="yew" disabled=DisabledState::Disabled>"Yew (disabled)"</SelectItem>
                    </SelectContent>
                </Select>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection, visibility and interaction fully governed by structure."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <Select>
                        <SelectTrigger>
                            <SelectValue placeholder="Select size...">"All Sizes"</SelectValue>
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value="sm">"Small"</SelectItem>
                            <SelectItem value="md" selected=SelectionState::Selected>"Medium"</SelectItem>
                            <SelectItem value="lg">"Large"</SelectItem>
                        </SelectContent>
                    </Select>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <Select disabled=DisabledState::Disabled>
                        <SelectTrigger>
                            <SelectValue placeholder="Disabled...">"Disabled"</SelectValue>
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
