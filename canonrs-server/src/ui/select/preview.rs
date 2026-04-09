use leptos::prelude::*;
use super::select_island::SelectIsland;
use super::select_ui::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn SelectShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SelectIsland placeholder="Choose a framework...">
                    <SelectItem value="leptos">"Leptos"</SelectItem>
                    <SelectItem value="dioxus">"Dioxus"</SelectItem>
                    <SelectItem value="yew" disabled=DisabledState::Disabled>"Yew (disabled)"</SelectItem>
                </SelectIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection, visibility and interaction fully governed by structure."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <Select>
                        <SelectTrigger>
                            <SelectValue placeholder="Select size...">"Medium"</SelectValue>
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
                <span data-rs-showcase-preview-label="">"With disabled options"</span>
                <div data-rs-showcase-preview-row="">
                    <SelectIsland placeholder="Select plan...">
                        <SelectItem value="free">"Free"</SelectItem>
                        <SelectItem value="pro">"Pro"</SelectItem>
                        <SelectItem value="enterprise" disabled=DisabledState::Disabled>"Enterprise (disabled)"</SelectItem>
                        <SelectItem value="custom" disabled=DisabledState::Disabled>"Custom (disabled)"</SelectItem>
                    </SelectIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Many options — keyboard nav"</span>
                <div data-rs-showcase-preview-row="">
                    <SelectIsland placeholder="Select country...">
                        <SelectItem value="br">"Brazil"</SelectItem>
                        <SelectItem value="us">"United States"</SelectItem>
                        <SelectItem value="de">"Germany"</SelectItem>
                        <SelectItem value="jp">"Japan"</SelectItem>
                        <SelectItem value="fr">"France"</SelectItem>
                        <SelectItem value="uk">"United Kingdom"</SelectItem>
                    </SelectIsland>
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
