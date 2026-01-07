use leptos::prelude::*;
use leptos::server;
use rs_design::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentToken {
    pub token_name: String,
    pub token_scope: String,
    pub token_category: String,
    pub token_value: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub id: i32,
    pub name: String,
    pub families: String,
    pub file_path: String,
}

#[server]
pub async fn get_all_components() -> Result<Vec<ComponentInfo>, ServerFnError> {
    use rusqlite::Connection;
    
    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let mut stmt = conn.prepare(
        "SELECT id, name, COALESCE(familias_aplicadas, ''), COALESCE(file_path, '') 
         FROM components 
         ORDER BY name"
    ).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let components = stmt.query_map([], |row| {
        Ok(ComponentInfo {
            id: row.get(0)?,
            name: row.get(1)?,
            families: row.get(2)?,
            file_path: row.get(3)?,
        })
    })
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(components)
}

#[server]
pub async fn get_component_tokens(component_name: String) -> Result<Vec<ComponentToken>, ServerFnError> {
    use rusqlite::Connection;
    
    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let mut all_tokens = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT category || '.' || name as full_name, scope, category, value, COALESCE(description, '') as desc
         FROM tokens 
         WHERE scope = 'canonical' 
         ORDER BY category, name"
    ).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let tokens = stmt.query_map([], |row| {
        Ok(ComponentToken {
            token_name: row.get(0)?,
            token_scope: row.get(1)?,
            token_category: row.get(2)?,
            token_value: row.get(3)?,
            description: row.get(4)?,
        })
    })
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    
    all_tokens.extend(tokens.collect::<Result<Vec<_>, _>>().map_err(|e| ServerFnError::new(e.to_string()))?);
    
    let families: String = conn.query_row(
        "SELECT COALESCE(familias_aplicadas, '') FROM components WHERE name = ?1",
        [&component_name],
        |row| row.get(0)
    ).unwrap_or_default();
    
    if !families.is_empty() {
        let mut stmt = conn.prepare(
            "SELECT category || '.' || name as full_name, scope, category, value, COALESCE(description, '') as desc
             FROM tokens 
             WHERE scope = 'contextual' 
             ORDER BY category, name"
        ).map_err(|e| ServerFnError::new(e.to_string()))?;
        
        let tokens = stmt.query_map([], |row| {
            Ok(ComponentToken {
                token_name: row.get(0)?,
                token_scope: row.get(1)?,
                token_category: row.get(2)?,
                token_value: row.get(3)?,
                description: row.get(4)?,
            })
        })
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
        all_tokens.extend(tokens.collect::<Result<Vec<_>, _>>().map_err(|e| ServerFnError::new(e.to_string()))?);
    }
    
    Ok(all_tokens)
}

#[server]
pub async fn get_component_source(component_name: String) -> Result<String, ServerFnError> {
    use rusqlite::Connection;
    use std::fs;
    
    let db_path = "/opt/docker/cores/monorepo-core/apps/core-auth/frontend-leptos/data/canonrs.db";
    let conn = Connection::open(db_path).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let file_path: String = conn.query_row(
        "SELECT COALESCE(file_path, '') FROM components WHERE name = ?1",
        [&component_name],
        |row| row.get(0)
    ).map_err(|e| ServerFnError::new(format!("Component not found: {}", e)))?;
    
    if file_path.is_empty() {
        return Ok(String::from("// No source file configured"));
    }
    
    let base_path = "../../crates/rs-design/src";
    let full_path = format!("{}{}", base_path, file_path);
    
    fs::read_to_string(&full_path)
        .map_err(|e| ServerFnError::new(format!("Could not read {}: {}", full_path, e)))
}

#[component]
pub fn ChecklistTab() -> impl IntoView {
    let selected_component = RwSignal::new(String::from("Button"));
    let components = Resource::new(|| (), |_| get_all_components());
    let tokens = Resource::new(
        move || selected_component.get(),
        |name| get_component_tokens(name),
    );
    let source_code = Resource::new(
        move || selected_component.get(),
        |name| get_component_source(name),
    );
    
    view! {
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
            <div class="lg:col-span-8 space-y-4">
                <div class="space-y-2">
                    <Label>"Select Component"</Label>
                    <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                        {move || {
                            components.get().map(|comps| {
                                match comps {
                                    Ok(list) => view! {
                                        <select 
                                            class="w-full rounded-md border border-border bg-background px-3 py-2"
                                            on:change=move |ev| {
                                                selected_component.set(event_target_value(&ev));
                                            }
                                        >
                                            {list.iter().map(|comp| {
                                                let comp_name = comp.name.clone();
                                                let families = comp.families.clone();
                                                let is_selected = comp_name == selected_component.get_untracked();
                                                view! {
                                                    <option value=comp_name.clone() selected=is_selected>
                                                        {comp_name.clone()} " (Families: " {families} ")"
                                                    </option>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </select>
                                    }.into_any(),
                                    Err(e) => view! {
                                        <div class="text-destructive">"Error: " {e.to_string()}</div>
                                    }.into_any(),
                                }
                            })
                        }}
                    </Suspense>
                </div>
                
                <div class="border rounded-lg">
                    <div class="bg-muted px-4 py-3 border-b">
                        <h3 class="font-semibold">"Component Tokens"</h3>
                    </div>
                    <div class="overflow-auto" style="max-height: 600px">
                        <Suspense fallback=|| view! { <div class="p-4">"Loading..."</div> }>
                            {move || {
                                tokens.get().map(|token_list| {
                                    match token_list {
                                        Ok(list) => view! {
                                            <table class="w-full">
                                                <thead class="bg-muted sticky top-0">
                                                    <tr>
                                                        <th class="px-4 py-2 text-left text-sm font-medium">"Scope"</th>
                                                        <th class="px-4 py-2 text-left text-sm font-medium">"Token"</th>
                                                        <th class="px-4 py-2 text-left text-sm font-medium">"Value"</th>
                                                        <th class="px-4 py-2 text-left text-sm font-medium">"Description"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {list.iter().map(|token| {
                                                        let type_badge = if token.token_scope == "canonical" {
                                                            "bg-primary text-primary-foreground"
                                                        } else {
                                                            "bg-accent text-accent-foreground"
                                                        };
                                                        
                                                        let token_name = token.token_name.clone();
                                                        let token_value = token.token_value.clone();
                                                        let description = token.description.clone();
                                                        let token_scope = token.token_scope.clone();
                                                        
                                                        view! {
                                                            <tr class="border-b hover:bg-muted/50">
                                                                <td class="px-4 py-3">
                                                                    <span class=format!("px-2 py-1 rounded text-xs font-medium {}", type_badge)>
                                                                        {token_scope}
                                                                    </span>
                                                                </td>
                                                                <td class="px-4 py-3 font-mono text-sm">{token_name}</td>
                                                                <td class="px-4 py-3 font-mono text-sm text-muted-foreground">{token_value}</td>
                                                                <td class="px-4 py-3 text-sm">{description}</td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </tbody>
                                            </table>
                                        }.into_any(),
                                        Err(e) => view! {
                                            <div class="p-4 text-destructive">"Error: " {e.to_string()}</div>
                                        }.into_any(),
                                    }
                                })
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
            
            <div class="lg:col-span-4">
                <div class="sticky top-4">
                    <div class="border rounded-lg bg-card">
                        <div class="bg-muted px-4 py-3 border-b flex items-center justify-between">
                            <h3 class="font-semibold">"Source Code"</h3>
                            <span class="text-xs text-muted-foreground font-mono">
                                {move || selected_component.get()}
                            </span>
                        </div>
                        <div class="overflow-auto" style="max-height: 800px">
                            <Suspense fallback=|| view! { <div class="p-4 text-sm text-muted-foreground">"Loading..."</div> }>
                                {move || {
                                    source_code.get().map(|code| {
                                        match code {
                                            Ok(source) => view! {
                                                <pre class="text-xs font-mono p-4 overflow-x-auto">
                                                    <code>{source}</code>
                                                </pre>
                                            }.into_any(),
                                            Err(e) => view! {
                                                <div class="p-4 text-sm text-muted-foreground">
                                                    {e.to_string()}
                                                </div>
                                            }.into_any(),
                                        }
                                    })
                                }}
                            </Suspense>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
