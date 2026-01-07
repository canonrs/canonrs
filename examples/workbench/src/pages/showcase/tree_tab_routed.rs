use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate, use_query_map};
use rs_design::ui::tree::{Tree, TreeNode};
use rs_design::ui::breadcrumb::{Breadcrumb, BreadcrumbItemData};
use rs_design::ui::command::CommandPalette;
use super::{SelectionContext, create_contextual_commands, url_sync::*};

#[component]
pub fn TreeTabRouted() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let query = use_query_map();
    
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
    
    // Selected ID from query param "path"
    let selected_id = Signal::derive(move || {
        let path_param = query.get().get("path").unwrap_or_default();
        get_selected_id_from_path(&path_param)
    });
    
    let (palette_open, set_palette_open) = signal(false);
    let (palette_mounted, set_palette_mounted) = signal(false);
    
    // Auto-expand tree based on query param on mount
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let path_param = query.get().get("path").unwrap_or_default();
        let parent_ids = get_parent_ids_from_path(&path_param);
        
        if !parent_ids.is_empty() {
            set_nodes.update(|nodes| {
                fn expand_nodes(node: &mut TreeNode, ids_to_expand: &[String]) {
                    if ids_to_expand.contains(&node.id) {
                        node.expanded = true;
                    }
                    for child in &mut node.children {
                        expand_nodes(child, ids_to_expand);
                    }
                }
                
                for node in nodes.iter_mut() {
                    expand_nodes(node, &parent_ids);
                }
            });
        }
    });
    
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
        if let Some(id) = selected_id.get() {
            let path = build_path(&nodes.get(), &id);
            let total = path.len();
            
            path.into_iter().enumerate().map(|(idx, node)| {
                let partial_path = build_path(&nodes.get(), &node.id);
                let ids: Vec<String> = partial_path.iter().map(|n| n.id.clone()).collect();
                let path_str = build_path_from_ids(&ids);
                let url = format!("/components?path={}", path_str);
                
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
    
    let selection_context = Signal::derive(move || {
        if let Some(id) = selected_id.get() {
            let node = nodes.get().iter()
                .find_map(|n| n.find(&id))
                .cloned();
            
            if let Some(node) = node {
                SelectionContext::with_selection(
                    node.id,
                    node.node_type,
                    node.label,
                    node.metadata,
                )
            } else {
                SelectionContext::new()
            }
        } else {
            SelectionContext::new()
        }
    });
    
    let on_action = Callback::new(move |action: String| {
        leptos::logging::log!("🎬 Action: {}", action);
        set_palette_open.set(false);
    });
    
    let contextual_commands = create_contextual_commands(selection_context, on_action);
    
    // Tree selection updates query param
    let on_select = Callback::new(move |id: String| {
        let path = build_path(&nodes.get(), &id);
        let ids: Vec<String> = path.iter().map(|n| n.id.clone()).collect();
        let path_str = build_path_from_ids(&ids);
        let url = format!("/components?path={}", path_str);
        navigate(&url, Default::default());
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
    
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-2xl font-bold mb-2">"Tree + Route Sync"</h2>
                    <p class="text-muted-foreground">"URL-driven navigation"</p>
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
                <Breadcrumb items=breadcrumb_items separator="›" />
            </div>
            
            <div class="grid grid-cols-3 gap-6">
                <div class="border rounded-lg overflow-hidden">
                    <div class="p-3 bg-muted border-b">
                        <h3 class="font-semibold text-sm">"Navigator"</h3>
                    </div>
                    <Tree
                        nodes=nodes
                        selected_id=selected_id
                        on_select=on_select
                        on_toggle=on_toggle
                    />
                </div>
                
                <div class="col-span-2 space-y-6">
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"URL State"</h3>
                        <div class="space-y-2">
                            <div>
                                <span class="text-sm text-muted-foreground">"Query: "</span>
                                <code class="text-sm">{move || query.get().get("path").unwrap_or("none".into())}</code>
                            </div>
                            <div>
                                <span class="text-sm text-muted-foreground">"Selected: "</span>
                                <code class="text-sm">{move || selected_id.get().unwrap_or_default()}</code>
                            </div>
                        </div>
                    </div>
                    
                    <div class="border rounded-lg p-6">
                        <h3 class="font-semibold mb-4">"Commands"</h3>
                        <div class="space-y-2">
                            {move || {
                                let commands = contextual_commands.get().get_all_commands();
                                commands.into_iter().map(|cmd| {
                                    view! {
                                        <div class="p-2 bg-muted/30 rounded text-sm">
                                            {cmd.label}
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="p-4 bg-green-50 border border-green-200 rounded">
                <p class="text-sm font-semibold text-green-900">"✅ Route Sync via Query Params"</p>
                <ul class="text-xs text-green-700 mt-2 space-y-1">
                    <li>"• URL: /components?path=/workflows/workflow-1/step-1-2"</li>
                    <li>"• Tree selection updates query param"</li>
                    <li>"• Breadcrumb links use query params"</li>
                    <li>"• Browser back/forward works"</li>
                    <li>"• Shareable URLs"</li>
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
