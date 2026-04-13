use leptos::prelude::*;
use super::command_boundary::{Command, CommandItem};
use crate::blocks::card::CardBlock;
use crate::ui::card::{CardHeader, CardTitle, CardContent};
use canonrs_core::slot;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CommandShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CardBlock
                header=slot!(|| view! {
                    <CardHeader><CardTitle>"Command Palette"</CardTitle></CardHeader>
                }.into_any())
                content=slot!(|| view! {
                    <CardContent>
                        <Command placeholder="Search commands...">
                            <CommandItem value="calendar" group="Suggestions">"Calendar"</CommandItem>
                            <CommandItem value="search"   group="Suggestions">"Search"</CommandItem>
                            <CommandItem value="settings" group="Suggestions">"Settings"</CommandItem>
                            <CommandItem value="new"      group="Actions">"New file"</CommandItem>
                            <CommandItem value="open"     group="Actions">"Open file"</CommandItem>
                            <CommandItem value="save"     group="Actions">"Save"</CommandItem>
                        </Command>
                    </CardContent>
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "Command palette — typeahead filter governed by DOM."
            </p>
        </Stack>
    }
}
