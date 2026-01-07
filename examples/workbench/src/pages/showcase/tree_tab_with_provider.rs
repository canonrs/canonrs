use leptos::prelude::*;
use rs_design::ui::tree::{Tree, TreeNode};
use rs_design::ui::breadcrumb::{Breadcrumb, BreadcrumbItemData};
use rs_design::ui::command::CommandPalette;
use super::{
    SelectionContextProvider,
    use_selection_context,
    InspectorPanel,
    create_contextual_commands,
    SelectionContext,
};

#[component]
pub fn TreeTabWithProvider() -> impl IntoView {
    view! {
        <SelectionContextProvider>
            <TreeTabContent />
        </SelectionContextProvider>
    }
}

#[component]
fn TreeTabContent() -> impl IntoView {
    // Acessa contexto via hook (sem prop drilling!)
    let ctx = use_selection_context();
    
    let (nodes, set_nodes) = signal(vec![
        TreeNode::new("workflows", "Workflows", "root")
            .with_icon("📋")
            .with_expanded(true)
            .with_children(vec![
                TreeNode::new("workflow-1", "User Onboarding", "workflow")
                    .with_icon("👤")
                    .with_expanded(true)
                    .with_children(vec![
                        TreeNode::new("step-1-1", "Review", "step")
                            .with_icon("✅")
                            .with_metadata("Completed"),
                        TreeNode::new("step-1-2", "Approval", "step")
                            .with_icon("⏳")
                            .with_metadata("Active"),
                        TreeNode::new("step-1-3", "Deploy", "step")
                            .with_icon("🔒")
                            .with_metadata("Blocked"),
                        TreeNode::new("step-1-4", "Monitoring", "step")
                            .with_icon("⏸️")
                            .with_metadata("Pending"),
                    ]),
                TreeNode::new("workflow-2", "Billing Flow", "workflow")
                    .with_icon("💰")
                    .with_children(vec![
                        TreeNode::new("step-2-1", "Validation", "step")
                            .with_icon("✅"),
                        TreeNode::new("step-2-2", "Charge", "step")
                            .with_icon("💳"),
                        TreeNode::new("step-2-3", "Receipt", "step")
                            .with_icon("📧"),
                    ]),
            ]),
        TreeNode::new("auth", "Authentication", "root")
            .with_icon("🔐")
            .with_children(vec![
                TreeNode::new("users", "Users", "collection")
                    .with_icon("👥")
                    .with_children(vec![
                        TreeNode::new("user-1", "admin@corp.com", "user")
                            .with_icon("👤"),
                        TreeNode::new("user-2", "user@test.com", "user")
                            .with_icon("👤"),
                    ]),
                TreeNode::new("roles", "Roles", "collection")
                    .with_icon("🎭"),
                TreeNode::new("permissions", "Permissions", "collection")
                    .with_icon("🔑"),
            ]),
    ]);
    
    let (palette_open, set_palette_open) = signal(false);
    let (palette_mounted, set_palette_mounted) = signal(false);
    
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        use wasm_bindgen::JsCast;
        
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
    
    fn build_path(nodes: &Vec<TreeNode>, target_id: &str) -> Vec<TreeNode> {
        fn search_path(node: &TreeNode, target_id: &str, path: &mut Vec<TreeNode>) -> bool {
            path.push(node.clone());
            
            if node.id == target_id {
                return true;
            }
            
            for child in &node.children {
                if search_path(child, target_id, path) {
                    return true;
                }
            }
            
            path.pop();
            false
        }
        
        let mut path = Vec::new();
        for node in nodes {
            if search_path(node, target_id, &mut path) {
                return path;
            }
        }
        vec![]
    }
    
    let breadcrumb_items = Signal::derive(move || {
        if let Some(id) = ctx.selected_id.get() {
            let path = build_path(&nodes.get(), &id);
            let total = path.len();
            
            path.into_iter().enumerate().map(|(idx, node)| {
                let partial_path = build_path(&nodes.get(), &node.id);
                let ids: Vec<String> = partial_path.iter().map(|n| n.id.clone()).collect();
                let url = format!("#{}", ids.join("/"));
                
                BreadcrumbItemData {
                    label: node.label,
                    href: Some(url),
                    is_current: idx == total - 1,
                }
            }).collect::<Vec<_>>()
        } else {
            vec![]
        }
    });
    
    // Criar SelectionContext para backwards compatibility com create_contextual_commands
    let selection_context = Signal::derive(move || {
        SelectionContext {
            selected_id: ctx.selected_id.get(),
            node_type: ctx.node_type.get(),
            label: ctx.label.get(),
            metadata: ctx.metadata.get(),
        }
    });
    
    let on_action = Callback::new(move |action: String| {
        leptos::logging::log!("🎬 Action: {}", action);
        set_palette_open.set(false);
    });
    
    let contextual_commands = create_contextual_commands(selection_context, on_action);
    
    let on_select = Callback::new(move |id: String| {
        let node = nodes.get().iter()
            .find_map(|n| n.find(&id))
            .cloned();
        
        if let Some(node) = node {
            ctx.update_selection(
                Some(node.id),
                Some(node.node_type),
                Some(node.label),
                node.metadata,
            );
        }
    });
    
    let on_toggle = Callback::new(move |id: String| {
        set_nodes.update(|nodes| {
            for node in nodes.iter_mut() {
                if let Some(found) = node.find_mut(&id) {
                    found.expanded = !found.expanded;
                    break;
                }
            }
        });
    });
    
    let on_breadcrumb_navigate = Callback::new(move |href: String| {
        let id = href.trim_start_matches('#').split('/').last().unwrap_or("");
        if !id.is_empty() {
            let node = nodes.get().iter()
                .find_map(|n| n.find(id))
                .cloned();
            
            if let Some(node) = node {
                ctx.update_selection(
                    Some(node.id),
                    Some(node.node_type),
                    Some(node.label),
                    node.metadata,
                );
            }
        }
    });
    
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-2xl font-bold mb-2">"Tree + Context Provider"</h2>
                    <p class="text-muted-foreground">"Zero prop drilling with Provider pattern"</p>
                </div>
                
                <button
                    class="px-4 py-2 text-sm rounded bg-primary text-primary-foreground hover:bg-primary/90 flex items-center gap-2"
                    on:click=move |_| set_palette_open.set(true)
                >
                    <span>"Commands"</span>
                    <kbd class="px-1.5 py-0.5 text-xs rounded bg-primary-foreground/20">"⌘K"</kbd>
                </button>
            </div>
            
            <div class="border-b pb-4">
                <Breadcrumb
                    items=breadcrumb_items
                    on_navigate=on_breadcrumb_navigate
                    separator="›"
                />
            </div>
            
            <div class="grid grid-cols-3 gap-6">
                <div class="border rounded-lg overflow-hidden">
                    <div class="p-3 bg-muted border-b">
                        <h3 class="font-semibold text-sm">"Navigator"</h3>
                    </div>
                    <Tree
                        nodes=nodes
                        selected_id=Signal::derive(move || ctx.selected_id.get())
                        on_select=on_select
                        on_toggle=on_toggle
                    />
                </div>
                
                <div class="col-span-2 space-y-6">
                    <InspectorPanel on_action=on_action />
                </div>
            </div>
            
            <div class="p-4 bg-purple-50 border border-purple-200 rounded">
                <p class="text-sm font-semibold text-purple-900">"✅ Context Provider Pattern"</p>
                <ul class="text-xs text-purple-700 mt-2 space-y-1">
                    <li>"• InspectorPanel uses use_selection_context() - no props!"</li>
                    <li>"• CommandPalette can access context directly"</li>
                    <li>"• Easy to add new consumers (StatusBar, ActionBar, etc)"</li>
                    <li>"• Single source of truth for selection"</li>
                    <li>"• Zero prop drilling"</li>
                </ul>
            </div>
            
            <Show when=move || palette_mounted.get()>
                <CommandPalette
                    registry=contextual_commands.get()
                    open=palette_open
                    on_close=Callback::new(move |_| set_palette_open.set(false))
                />
            </Show>
        </div>
    }
}
