use leptos::prelude::*;
use super::{CommandInput, CommandList, command_types::{Command, CommandRegistry}};

/// CommandPalette - Main command palette component
/// 
/// **Type:** Stateful Component (Type 2)
/// **Pattern:** Input → Filter → Execute
/// **Tokens:** 100% Canonical
#[component]
pub fn CommandPalette(
    /// Registry of available commands
    registry: CommandRegistry,
    /// Whether palette is open
    #[prop(into)]
    open: Signal<bool>,
    /// Callback to close palette
    #[prop(into)]
    on_close: Callback<()>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0usize);
    
    // Filtered commands based on query
    let filtered_commands = Signal::derive(move || {
        let q = query.get();
        if q.is_empty() {
            registry.get_all_commands()
        } else {
            registry.search(&q)
        }
    });
    
    // Reset state when opened
    Effect::new(move |_| {
        if open.get() {
            set_query.set(String::new());
            set_selected_index.set(0);
        }
    });
    
    // Execute command
    let execute_command = move |cmd: Command| {
        cmd.callback.call();
        on_close.run(());
    };
    
    // Keyboard navigation
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        match key.as_str() {
            "Escape" => {
                ev.prevent_default();
                on_close.run(());
            }
            "ArrowDown" => {
                ev.prevent_default();
                set_selected_index.update(|idx| {
                    let max = filtered_commands.get().len().saturating_sub(1);
                    *idx = (*idx + 1).min(max);
                });
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| {
                    *idx = idx.saturating_sub(1);
                });
            }
            "Enter" => {
                ev.prevent_default();
                let cmds = filtered_commands.get();
                let idx = selected_index.get();
                if let Some(cmd) = cmds.get(idx) {
                    execute_command(cmd.clone());
                }
            }
            _ => {}
        }
    };
    
    view! {
        <Show when=move || open.get()>
            <div
                class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh] bg-black/50"
                on:click=move |_| on_close.run(())
            >
                <div
                    class="w-full max-w-2xl bg-background border border-border rounded-lg shadow-lg"
                    on:click=|ev| ev.stop_propagation()
                    on:keydown=on_keydown
                >
                    <CommandInput
                        value=query
                        on_input=Callback::new(move |val: String| {
                            set_query.set(val);
                            set_selected_index.set(0);
                        })
                    />
                    
                    <CommandList
                        commands=filtered_commands
                        selected_index=selected_index
                        on_select=Callback::new(execute_command)
                    />
                </div>
            </div>
        </Show>
    }
}
