use leptos::prelude::*;
use super::{CommandPanelBlock, CommandPanelItem};

pub fn basic_example() -> impl IntoView {
    view! {
        <CommandPanelBlock open=Signal::derive(|| true)>
            <CommandPanelItem label="New Document".to_string() shortcut="⌘N".to_string() />
            <CommandPanelItem label="Open File".to_string() shortcut="⌘O".to_string() />
            <CommandPanelItem label="Save".to_string() shortcut="⌘S".to_string() />
        </CommandPanelBlock>
    }
}
