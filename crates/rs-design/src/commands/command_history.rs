use leptos::prelude::*;
use super::command_trait::Command;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct CommandHistory {
    undo_stack: RwSignal<Vec<Arc<dyn Command>>>,
    redo_stack: RwSignal<Vec<Arc<dyn Command>>>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: RwSignal::new(Vec::new()),
            redo_stack: RwSignal::new(Vec::new()),
        }
    }
    
    pub fn execute(&self, command: impl Command + 'static) {
        leptos::logging::log!("📝 Executing: {}", command.description());
        command.execute();
        
        self.undo_stack.update(|stack| stack.push(Arc::new(command)));
        self.redo_stack.update(|stack| stack.clear());
    }
    
    pub fn undo(&self) -> Option<String> {
        // Get command from stack
        let cmd = self.undo_stack.with(|stack| stack.last().cloned())?;
        
        // Remove from undo stack
        self.undo_stack.update(|stack| { stack.pop(); });
        
        // Execute undo
        leptos::logging::log!("↩️  Undoing: {}", cmd.description());
        cmd.undo();
        
        // Add to redo stack
        let desc = cmd.description();
        self.redo_stack.update(|stack| stack.push(cmd));
        
        Some(desc)
    }
    
    pub fn redo(&self) -> Option<String> {
        // Get command from redo stack
        let cmd = self.redo_stack.with(|stack| stack.last().cloned())?;
        
        // Remove from redo stack
        self.redo_stack.update(|stack| { stack.pop(); });
        
        // Execute redo
        leptos::logging::log!("↪️  Redoing: {}", cmd.description());
        cmd.execute();
        
        // Add to undo stack
        let desc = cmd.description();
        self.undo_stack.update(|stack| stack.push(cmd));
        
        Some(desc)
    }
    
    pub fn can_undo(&self) -> Signal<bool> {
        let undo = self.undo_stack;
        Signal::derive(move || !undo.get().is_empty())
    }
    
    pub fn can_redo(&self) -> Signal<bool> {
        let redo = self.redo_stack;
        Signal::derive(move || !redo.get().is_empty())
    }
    
    pub fn undo_history(&self) -> Signal<Vec<String>> {
        let undo = self.undo_stack;
        Signal::derive(move || {
            undo.get().iter().map(|c| c.description()).collect()
        })
    }
}

#[component]
pub fn CommandHistoryProvider(children: Children) -> impl IntoView {
    leptos::logging::log!("🏗️  CommandHistoryProvider created");
    let history = CommandHistory::new();
    provide_context(history);
    
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        
        let history_kb = history;
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.ctrl_key() || event.meta_key() {
                match event.key().as_str() {
                    "z" if !event.shift_key() => {
                        event.prevent_default();
                        history_kb.undo();
                    }
                    "y" | "Z" => {
                        event.prevent_default();
                        history_kb.redo();
                    }
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "keydown",
                closure.as_ref().unchecked_ref()
            );
        }
        
        closure.forget();
    }
    
    children()
}

pub fn use_command_history() -> CommandHistory {
    expect_context::<CommandHistory>()
}
