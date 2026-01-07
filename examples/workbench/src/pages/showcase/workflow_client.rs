use leptos::prelude::*;
use super::{WorkflowView, WorkflowActions};
use super::database_server::fetch_workflow_steps;

#[component]
pub fn WorkflowClient() -> impl IntoView {
    let (refresh, set_refresh) = signal(0u32);
    let (palette_mounted, set_palette_mounted) = signal(false);
    let (palette_open, set_palette_open) = signal(false);
    
    let steps_for_actions = Resource::new(
        move || refresh.get(),
        |_| async { fetch_workflow_steps(1).await }
    );
    
    let on_transition = Callback::new(move |_: ()| {
        set_refresh.update(|n| *n += 1);
    });
    
    // Montar palette APÓS hydration
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::JsCast;
        
        // Atalho Ctrl+K
        let handler = move |ev: web_sys::KeyboardEvent| {
            if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "k" {
                ev.prevent_default();
                set_palette_open.update(|open| *open = !*open);
            }
        };
        
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
        
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        }
        
        closure.forget();
        set_palette_mounted.set(true);
    });
    
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-2xl font-bold mb-2">"Workflow (CQRS Persisted)"</h2>
                    <p class="text-sm text-muted-foreground">"View (SSR) + Actions (Client) + Commands"</p>
                </div>
                
                <button
                    class="px-4 py-2 text-sm rounded bg-primary text-primary-foreground hover:bg-primary/90 flex items-center gap-2"
                    on:click=move |_| set_palette_open.set(true)
                >
                    <span>"Commands"</span>
                    <kbd class="px-1.5 py-0.5 text-xs rounded bg-primary-foreground/20">"⌘K"</kbd>
                </button>
            </div>
            
            <WorkflowView refresh_trigger=refresh />
            
            <div class="border-t pt-4 mt-6">
                <h3 class="font-semibold mb-3">"Actions"</h3>
                <Transition fallback=|| view! { <div>"Loading..."</div> }>
                    {move || steps_for_actions.get().map(|result| match result {
                        Ok(steps) => view! {
                            <div class="space-y-2">
                                {steps.into_iter().map(|step| {
                                    view! {
                                        <div class="flex items-center justify-between p-2 bg-muted/30 rounded">
                                            <span class="text-sm font-medium">{step.label}</span>
                                            <WorkflowActions
                                                step_id=step.step_id
                                                current_status=step.status
                                                on_transition_complete=on_transition
                                            />
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any(),
                        Err(_) => view! { <div>"Error"</div> }.into_any()
                    })}
                </Transition>
            </div>
            
            <Show when=move || palette_mounted.get()>
                {move || {
                    #[cfg(target_arch = "wasm32")]
                    {
                        use super::workflow_commands::create_workflow_commands;
                        use rs_design::ui::command::CommandPalette;
                        
                        let command_registry = create_workflow_commands(on_transition);
                        view! {
                            <CommandPalette
                                registry=command_registry
                                open=palette_open
                                on_close=Callback::new(move |_| set_palette_open.set(false))
                            />
                        }.into_any()
                    }
                    
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        view! { <></> }.into_any()
                    }
                }}
            </Show>
        </div>
    }
}
