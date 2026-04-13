use leptos::prelude::*;
use super::command_boundary::{Command, CommandItem};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CommandShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Command placeholder="Search commands...">
                <CommandItem value="calendar" group="Suggestions">"Calendar"</CommandItem>
                <CommandItem value="search"   group="Suggestions">"Search"</CommandItem>
                <CommandItem value="settings" group="Suggestions">"Settings"</CommandItem>
                <CommandItem value="new"      group="Actions">"New file"</CommandItem>
                <CommandItem value="open"     group="Actions">"Open file"</CommandItem>
                <CommandItem value="save"     group="Actions">"Save"</CommandItem>
            </Command>
            <p data-rs-showcase-preview-anchor="">
                "Command palette — typeahead filter governed by DOM."
            </p>
        </Stack>
    }
}
