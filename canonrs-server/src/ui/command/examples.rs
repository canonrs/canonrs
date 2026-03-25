use leptos::prelude::*;
use super::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Command>
            <CommandInput placeholder="Search..." />
            <CommandList>
                <CommandItem>"Copy"</CommandItem>
                <CommandItem>"Cut"</CommandItem>
                <CommandItem>"Paste"</CommandItem>
                <CommandItem>"Delete"</CommandItem>
                <CommandItem>"Rename"</CommandItem>
            </CommandList>
        </Command>
    }
}
