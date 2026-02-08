use leptos::prelude::*;
use super::{Command, CommandInput, CommandList, CommandItem, command_types::CommandRegistry};

#[component]
pub fn CommandPalette(
    registry: CommandRegistry,
    open: Signal<bool>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);
    let registry_stored = StoredValue::new(registry);

    let filtered_commands = move || {
        let q = query.get();
        registry_stored.with_value(|reg| {
            if q.is_empty() {
                reg.get_all_commands()
            } else {
                reg.search(&q)
            }
        })
    };

    Effect::new(move |_| {
        if open.get() {
            set_query.set(String::new());
            set_selected_index.set(0);
        }
    });

    view! {
        {move || open.get().then(|| view! {
            <div class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh] bg-black/50">
                <Command>
                    <div class="w-full max-w-2xl bg-background border border-border rounded-lg shadow-lg">
                        <CommandInput
                            value=query.into()
                            placeholder="Type a command..."
                        />
                        
                        <CommandList
                            commands=Signal::derive(filtered_commands)
                            selected_index=selected_index.into()
                        />
                    </div>
                </Command>
            </div>
        })}
    }
}
