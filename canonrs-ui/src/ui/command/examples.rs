use leptos::prelude::*;
use super::{Command, CommandInputSimple};

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Command>
            <CommandInputSimple placeholder="Type a command...".to_string() />
        </Command>
    }
}
