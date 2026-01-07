use leptos::prelude::*;
use super::use_selection_context;

/// InspectorPanel - Displays details of selected node
#[component]
pub fn InspectorPanel(
    /// Optional callback for actions
    #[prop(optional)]
    on_action: Option<Callback<String>>,
) -> impl IntoView {
    let ctx = use_selection_context();
    
    let handle_action = move |action: String| {
        leptos::logging::log!("🎬 Inspector action: {}", action);
        if let Some(callback) = on_action {
            callback.run(action);
        }
    };
    
    view! {
        <div class="border rounded-lg overflow-hidden">
            <div class="p-3 bg-muted border-b">
                <h3 class="font-semibold text-sm">"Inspector"</h3>
            </div>
            
            <div class="p-6">
                {move || {
                    if ctx.has_selection() {
                        let selected_id = ctx.selected_id.get().unwrap_or_default();
                        
                        view! {
                            <div class="space-y-4">
                                <div>
                                    <div class="text-xs text-muted-foreground uppercase tracking-wide mb-1">
                                        "Selected Node"
                                    </div>
                                    <div class="font-semibold text-lg">
                                        {move || ctx.label.get().unwrap_or_default()}
                                    </div>
                                </div>
                                
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <div class="text-xs text-muted-foreground uppercase tracking-wide mb-1">
                                            "ID"
                                        </div>
                                        <code class="text-sm bg-muted px-2 py-1 rounded">
                                            {move || ctx.selected_id.get().unwrap_or_default()}
                                        </code>
                                    </div>
                                    
                                    <div>
                                        <div class="text-xs text-muted-foreground uppercase tracking-wide mb-1">
                                            "Type"
                                        </div>
                                        <code class="text-sm bg-muted px-2 py-1 rounded">
                                            {move || ctx.node_type.get().unwrap_or_default()}
                                        </code>
                                    </div>
                                </div>
                                
                                {move || ctx.metadata.get().map(|meta| {
                                    view! {
                                        <div>
                                            <div class="text-xs text-muted-foreground uppercase tracking-wide mb-1">
                                                "Metadata"
                                            </div>
                                            <code class="text-sm bg-muted px-2 py-1 rounded block">
                                                {meta}
                                            </code>
                                        </div>
                                    }
                                })}
                                
                                <div class="pt-4 border-t space-y-2">
                                    <div class="text-xs text-muted-foreground uppercase tracking-wide mb-2">
                                        "Context Actions"
                                    </div>
                                    
                                    {move || {
                                        let id = selected_id.clone();
                                        
                                        if ctx.is_type("step") {
                                            let id1 = id.clone();
                                            let id2 = id.clone();
                                            let id3 = id.clone();
                                            
                                            view! {
                                                <div class="space-y-2">
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-green-100 text-green-900 rounded hover:bg-green-200"
                                                        on:click=move |_| handle_action(format!("complete:{}", id1))
                                                    >
                                                        "✅ Complete Step"
                                                    </button>
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-red-100 text-red-900 rounded hover:bg-red-200"
                                                        on:click=move |_| handle_action(format!("fail:{}", id2))
                                                    >
                                                        "❌ Fail Step"
                                                    </button>
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-blue-100 text-blue-900 rounded hover:bg-blue-200"
                                                        on:click=move |_| handle_action(format!("reset:{}", id3))
                                                    >
                                                        "🔄 Reset Step"
                                                    </button>
                                                </div>
                                            }.into_any()
                                        } else if ctx.is_type("workflow") {
                                            let id1 = id.clone();
                                            let id2 = id.clone();
                                            
                                            view! {
                                                <div class="space-y-2">
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90"
                                                        on:click=move |_| handle_action(format!("add_step:{}", id1))
                                                    >
                                                        "➕ Add Step"
                                                    </button>
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-muted text-foreground rounded hover:bg-muted/80"
                                                        on:click=move |_| handle_action(format!("archive:{}", id2))
                                                    >
                                                        "📦 Archive Workflow"
                                                    </button>
                                                </div>
                                            }.into_any()
                                        } else if ctx.is_type("user") {
                                            let id1 = id.clone();
                                            let id2 = id.clone();
                                            
                                            view! {
                                                <div class="space-y-2">
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90"
                                                        on:click=move |_| handle_action(format!("edit_user:{}", id1))
                                                    >
                                                        "✏️ Edit User"
                                                    </button>
                                                    <button 
                                                        class="w-full px-3 py-2 text-sm bg-red-100 text-red-900 rounded hover:bg-red-200"
                                                        on:click=move |_| handle_action(format!("disable_user:{}", id2))
                                                    >
                                                        "🚫 Disable User"
                                                    </button>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <p class="text-sm text-muted-foreground italic">
                                                    "No actions available"
                                                </p>
                                            }.into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-center py-12">
                                <div class="text-4xl mb-4">"🔍"</div>
                                <p class="text-muted-foreground">"Select a node to inspect"</p>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
