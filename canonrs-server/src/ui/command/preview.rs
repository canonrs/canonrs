use leptos::prelude::*;
use super::command_island::{CommandIsland, CommandItemIsland};

#[component]
pub fn CommandShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CommandIsland placeholder="Search commands...">
                    <CommandItemIsland value="calendar" group="Suggestions">"Calendar"</CommandItemIsland>
                    <CommandItemIsland value="search" group="Suggestions">"Search"</CommandItemIsland>
                    <CommandItemIsland value="settings" group="Suggestions">"Settings"</CommandItemIsland>
                    <CommandItemIsland value="new" group="Actions">"New file"</CommandItemIsland>
                    <CommandItemIsland value="open" group="Actions">"Open file"</CommandItemIsland>
                    <CommandItemIsland value="save" group="Actions">"Save"</CommandItemIsland>
                </CommandIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Command palette — typeahead filter governed by DOM."
            </p>
        </div>
    }
}
