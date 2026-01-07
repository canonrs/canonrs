use leptos::prelude::*;

#[derive(Clone, Debug)]
struct HistoryRecord {
    description: String,
    old_value: i32,
    new_value: i32,
}

#[component]
pub fn CommandHistoryTab() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let (undo_stack, set_undo_stack) = signal(Vec::<HistoryRecord>::new());
    let (redo_stack, set_redo_stack) = signal(Vec::<HistoryRecord>::new());
    let (log, set_log) = signal(Vec::<String>::new());
    
    let increment = move || {
        let old = counter.get();
        let new = old + 1;
        set_counter.set(new);
        
        set_undo_stack.update(|s| {
            s.push(HistoryRecord {
                description: format!("Increment {} → {}", old, new),
                old_value: old,
                new_value: new,
            });
            if s.len() > 50 { s.remove(0); }
        });
        
        set_redo_stack.update(|s| s.clear());
        
        set_log.update(|l| {
            l.insert(0, format!("✅ Incremented to {}", new));
            if l.len() > 10 { l.truncate(10); }
        });
    };
    
    let decrement = move || {
        let old = counter.get();
        let new = old - 1;
        set_counter.set(new);
        
        set_undo_stack.update(|s| {
            s.push(HistoryRecord {
                description: format!("Decrement {} → {}", old, new),
                old_value: old,
                new_value: new,
            });
            if s.len() > 50 { s.remove(0); }
        });
        
        set_redo_stack.update(|s| s.clear());
        
        set_log.update(|l| {
            l.insert(0, format!("⬇️ Decremented to {}", new));
            if l.len() > 10 { l.truncate(10); }
        });
    };
    
    let undo = move || {
        let record = {
            let mut r = None;
            set_undo_stack.update(|s| { r = s.pop(); });
            r
        };
        
        if let Some(rec) = record {
            set_counter.set(rec.old_value);
            set_redo_stack.update(|s| s.push(rec.clone()));
            set_log.update(|l| {
                l.insert(0, format!("↩️ Undo: {}", rec.description));
                if l.len() > 10 { l.truncate(10); }
            });
        }
    };
    
    let redo = move || {
        let record = {
            let mut r = None;
            set_redo_stack.update(|s| { r = s.pop(); });
            r
        };
        
        if let Some(rec) = record {
            set_counter.set(rec.new_value);
            set_undo_stack.update(|s| s.push(rec.clone()));
            set_log.update(|l| {
                l.insert(0, format!("🔄 Redo: {}", rec.description));
                if l.len() > 10 { l.truncate(10); }
            });
        }
    };

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Command History (Undo/Redo)"</h2>
                <p class="text-muted-foreground">"Click buttons to test undo/redo"</p>
            </div>

            <div class="grid grid-cols-3 gap-6">
                <div class="col-span-2 space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Interactive Counter"</h3>
                        
                        <div class="flex items-center justify-center gap-6 py-8">
                            <button
                                class="px-6 py-3 rounded-lg bg-red-500 text-white hover:bg-red-600 text-2xl font-bold"
                                on:click=move |_| decrement()
                            >
                                "-"
                            </button>
                            
                            <div class="text-6xl font-bold text-primary">
                                {move || counter.get()}
                            </div>
                            
                            <button
                                class="px-6 py-3 rounded-lg bg-green-500 text-white hover:bg-green-600 text-2xl font-bold"
                                on:click=move |_| increment()
                            >
                                "+"
                            </button>
                        </div>
                    </div>

                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"History Controls"</h3>
                        
                        <div class="flex gap-4">
                            <button
                                class="flex-1 px-4 py-3 rounded-lg border-2 border-primary text-primary hover:bg-primary hover:text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
                                disabled=move || undo_stack.get().is_empty()
                                on:click=move |_| undo()
                            >
                                "↩️ Undo"
                            </button>
                            
                            <button
                                class="flex-1 px-4 py-3 rounded-lg border-2 border-primary text-primary hover:bg-primary hover:text-primary-foreground disabled:opacity-50 disabled:cursor-not-allowed"
                                disabled=move || redo_stack.get().is_empty()
                                on:click=move |_| redo()
                            >
                                "🔄 Redo"
                            </button>
                        </div>
                        
                        <div class="mt-4 grid grid-cols-2 gap-4 text-center">
                            <div class="p-3 bg-muted rounded">
                                <div class="text-2xl font-bold">{move || undo_stack.get().len()}</div>
                                <div class="text-sm text-muted-foreground">"Undo Stack"</div>
                            </div>
                            <div class="p-3 bg-muted rounded">
                                <div class="text-2xl font-bold">{move || redo_stack.get().len()}</div>
                                <div class="text-sm text-muted-foreground">"Redo Stack"</div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Event Log"</h3>
                        <div class="space-y-2 text-sm font-mono max-h-[400px] overflow-y-auto">
                            {move || log.get().into_iter().map(|msg| {
                                view! { <div class="text-xs">{msg}</div> }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="p-4 bg-green-50 border border-green-200 rounded">
                        <p class="text-sm font-semibold text-green-900">"✅ Proof of Concept"</p>
                        <ul class="text-xs text-green-700 mt-2 space-y-1">
                            <li>"• Undo/Redo stack"</li>
                            <li>"• Command history"</li>
                            <li>"• Max 50 commands"</li>
                        </ul>
                    </div>

                    <div class="p-4 bg-yellow-50 border border-yellow-200 rounded">
                        <p class="text-sm font-semibold text-yellow-900">"⚠️ Next: Add Keyboard"</p>
                        <ul class="text-xs text-yellow-700 mt-2 space-y-1">
                            <li>"• Ctrl+Z for undo"</li>
                            <li>"• Ctrl+Shift+Z for redo"</li>
                            <li>"• Global provider"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
