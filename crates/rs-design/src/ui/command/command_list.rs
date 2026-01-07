use leptos::prelude::*;
use super::{CommandItem, command_types::Command};

/// CommandList - List of filtered commands
/// 
/// **Type:** Pure Component (Type 1)
/// **Tokens:** 100% Canonical
#[component]
pub fn CommandList(
    /// Commands to display
    #[prop(into)]
    commands: Signal<Vec<Command>>,
    /// Currently selected index
    #[prop(into)]
    selected_index: Signal<usize>,
    /// Callback when command is selected
    #[prop(into)]
    on_select: Callback<Command>,
) -> impl IntoView {
    view! {
        <div class="max-h-96 overflow-y-auto">
            {move || {
                let cmds = commands.get();
                if cmds.is_empty() {
                    view! {
                        <div class="px-4 py-8 text-center text-muted-foreground">
                            "No commands found"
                        </div>
                    }.into_any()
                } else {
                    cmds.into_iter()
                        .enumerate()
                        .map(|(idx, cmd)| {
                            let cmd_clone = cmd.clone();
                            view! {
                                <CommandItem
                                    command=cmd
                                    selected=Signal::derive(move || selected_index.get() == idx)
                                    on_select=Callback::new(move |_| on_select.run(cmd_clone.clone()))
                                />
                            }
                        })
                        .collect_view()
                        .into_any()
                }
            }}
        </div>
    }
}
