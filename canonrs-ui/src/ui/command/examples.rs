use leptos::prelude::*;
use super::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Command id="command-example">
            <CommandItem text="Copy" />
            <CommandItem text="Cut" />
            <CommandItem text="Paste" />
            <CommandItem text="Delete" />
            <CommandItem text="Rename" />
        </Command>
    }
}
