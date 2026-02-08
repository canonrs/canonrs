use leptos::prelude::*;
use super::command_types::Command;

#[component]
pub fn CommandItem(
    command: Command,
    selected: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let base_class = if selected {
        "px-4 py-3 cursor-pointer transition-colors bg-primary/10 border-l-2 border-primary"
    } else {
        "px-4 py-3 cursor-pointer transition-colors hover:bg-muted"
    };

    let full_class = class.map(|c| format!("{} {}", base_class, c)).unwrap_or_else(|| base_class.to_string());

    view! {
        <div
            class={full_class}
            id={id}
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    {command.icon.map(|icon| view! {
                        <span class="text-lg">{icon}</span>
                    })}
                    <div>
                        <div class="font-medium">{command.label.clone()}</div>
                        {command.group.map(|group| view! {
                            <div class="text-xs text-muted-foreground">{group}</div>
                        })}
                    </div>
                </div>
                {command.shortcut.map(|shortcut| view! {
                    <kbd class="px-2 py-1 text-xs rounded bg-muted border border-border">
                        {shortcut}
                    </kbd>
                })}
            </div>
        </div>
    }
}
