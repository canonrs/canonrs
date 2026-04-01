use leptos::prelude::*;
use crate::ui::scroll_area::scroll_area_ui::ScrollArea;
use super::command_ui::{
    Command, CommandInput, CommandList, CommandGroup,
    CommandItem, CommandSeparator, CommandEmpty,
};
use canonrs_core::meta::SelectionState;

#[component]
pub fn CommandShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Command>
                    <CommandInput placeholder="Search commands..." />
                    <CommandList>
                        <ScrollArea>
                        <CommandGroup heading="Suggestions">
                            <CommandItem value="calendar">"Calendar"</CommandItem>
                            <CommandItem value="search" selected=SelectionState::Selected>"Search"</CommandItem>
                            <CommandItem value="settings">"Settings"</CommandItem>
                        </CommandGroup>
                        <CommandSeparator />
                        <CommandGroup heading="Actions">
                            <CommandItem value="new">"New file"</CommandItem>
                            <CommandItem value="open">"Open file"</CommandItem>
                            <CommandItem value="save">"Save"</CommandItem>
                        </CommandGroup>
                        </ScrollArea>
                    </CommandList>
                </Command>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Command palette semantics and selection fully enforced."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Empty state"</span>
                <div data-rs-showcase-preview-row="">
                    <Command>
                        <CommandInput placeholder="Type to search..." />
                        <CommandList>
                            <CommandEmpty>"No results found."</CommandEmpty>
                        </CommandList>
                    </Command>
                </div>
            </div>
        </div>
    }
}
