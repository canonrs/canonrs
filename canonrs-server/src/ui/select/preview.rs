use leptos::prelude::*;
use super::select_boundary::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem};
use canonrs_core::meta::{DisabledState, SelectionState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn SelectShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Selection, visibility and interaction fully governed by structure."
            </p>
            <Select placeholder="Choose a framework...">
                <SelectItem value="leptos">"Leptos"</SelectItem>
                <SelectItem value="dioxus">"Dioxus"</SelectItem>
                <SelectItem value="yew" disabled=DisabledState::Disabled>"Yew (disabled)"</SelectItem>
            </Select>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
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
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With disabled options"</span>
                <Select placeholder="Select plan...">
                    <SelectItem value="free">"Free"</SelectItem>
                    <SelectItem value="pro">"Pro"</SelectItem>
                    <SelectItem value="enterprise" disabled=DisabledState::Disabled>"Enterprise (disabled)"</SelectItem>
                    <SelectItem value="custom" disabled=DisabledState::Disabled>"Custom (disabled)"</SelectItem>
                </Select>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Many options — keyboard nav"</span>
                <Select placeholder="Select country...">
                    <SelectItem value="br">"Brazil"</SelectItem>
                    <SelectItem value="us">"United States"</SelectItem>
                    <SelectItem value="de">"Germany"</SelectItem>
                    <SelectItem value="jp">"Japan"</SelectItem>
                    <SelectItem value="fr">"France"</SelectItem>
                    <SelectItem value="uk">"United Kingdom"</SelectItem>
                </Select>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <Select disabled=DisabledState::Disabled>
                    <SelectTrigger disabled=DisabledState::Disabled>
                        <SelectValue placeholder="Disabled...">{""}</SelectValue>
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="a">"Option A"</SelectItem>
                    </SelectContent>
                </Select>
            </Stack>
        </Stack>
    }
}
