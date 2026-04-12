use leptos::prelude::*;
use super::boundary::{Command, CommandItem};

#[component]
pub fn CommandShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Command placeholder="Search commands...">
                    <CommandItem value="calendar" group="Suggestions">"Calendar"</CommandItem>
                    <CommandItem value="search" group="Suggestions">"Search"</CommandItem>
                    <CommandItem value="settings" group="Suggestions">"Settings"</CommandItem>
                    <CommandItem value="new" group="Actions">"New file"</CommandItem>
                    <CommandItem value="open" group="Actions">"Open file"</CommandItem>
                    <CommandItem value="save" group="Actions">"Save"</CommandItem>
                </Command>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Command palette — typeahead filter governed by DOM."
            </p>
        </div>
    }
}
