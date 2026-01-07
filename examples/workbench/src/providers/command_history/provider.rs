use leptos::prelude::*;
use super::command_history::CommandHistory;

/// CommandHistoryProvider - Provides command history context
#[component]
pub fn CommandHistoryProvider(
    /// Max history size
    #[prop(default = 50)]
    max_size: usize,
    
    /// Children
    children: Children,
) -> impl IntoView {
    let history = CommandHistory::new(max_size);
    
    provide_context(history);
    
    // Keyboard shortcuts (Ctrl+Z, Ctrl+Shift+Z) - ONLY on client
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;
        
        Effect::new(move |_| {
            let handler = move |ev: web_sys::KeyboardEvent| {
                let ctrl_or_meta = ev.ctrl_key() || ev.meta_key();
                
                if ctrl_or_meta && ev.key() == "z" {
                    if ev.shift_key() {
                        // Ctrl+Shift+Z = Redo
                        ev.prevent_default();
                        if let Some(_record) = history.redo() {
                            leptos::logging::log!("🔄 Redo");
                        }
                    } else {
                        // Ctrl+Z = Undo
                        ev.prevent_default();
                        if let Some(_record) = history.undo() {
                            leptos::logging::log!("↩️ Undo");
                        }
                    }
                }
            };
            
            let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
            
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback(
                    "keydown",
                    closure.as_ref().unchecked_ref()
                );
            }
            
            closure.forget();
        });
    }
    
    children()
}

/// Hook to access command history
pub fn use_command_history() -> CommandHistory {
    expect_context::<CommandHistory>()
}
