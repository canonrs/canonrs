use leptos::prelude::*;
use super::command_types::Command;

/// CommandItem - Single command in the list
/// 
/// **Type:** Pure Component (Type 1)
/// **Tokens:** 100% Canonical
#[component]
pub fn CommandItem(
    /// Command to display
    command: Command,
    /// Whether this item is selected
    #[prop(into)]
    selected: Signal<bool>,
    /// Callback when clicked
    #[prop(into)]
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "px-4 py-3 cursor-pointer transition-colors {}",
                if selected.get() {
                    "bg-primary/10 border-l-2 border-primary"
                } else {
                    "hover:bg-muted"
                }
            )
            on:click=move |_| on_select.run(())
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    {command.icon.clone().map(|icon| view! {
                        <span class="text-lg">{icon}</span>
                    })}
                    <div>
                        <div class="font-medium">{command.label.clone()}</div>
                        {command.group.clone().map(|group| view! {
                            <div class="text-xs text-muted-foreground">{group}</div>
                        })}
                    </div>
                </div>
                {command.shortcut.clone().map(|shortcut| view! {
                    <kbd class="px-2 py-1 text-xs rounded bg-muted border border-border">
                        {shortcut}
                    </kbd>
                })}
            </div>
        </div>
    }
}
