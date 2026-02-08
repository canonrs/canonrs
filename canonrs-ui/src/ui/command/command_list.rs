use leptos::prelude::*;
use super::{CommandItem, command_types::Command};

#[component]
pub fn CommandList(
    commands: Signal<Vec<Command>>,
    selected_index: Signal<usize>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let base_class = "max-h-96 overflow-y-auto";
    let full_class = class.map(|c| format!("{} {}", base_class, c)).unwrap_or_else(|| base_class.to_string());

    view! {
        <div class={full_class} id={id}>
            {move || {
                let cmds = commands.get();
                if cmds.is_empty() {
                    view! {
                        <div class="px-4 py-8 text-center text-muted-foreground">
                            "No commands found"
                        </div>
                    }.into_any()
                } else {
                    cmds.into_iter().enumerate().map(|(idx, cmd)| {
                        view! {
                            <CommandItem
                                command=cmd
                                selected={selected_index.get() == idx}
                            />
                        }
                    }).collect_view().into_any()
                }
            }}
        </div>
    }
}
