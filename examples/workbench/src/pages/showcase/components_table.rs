use leptos::prelude::*;
use leptos::prelude::Callback;
use rs_design::{DataTable, DataTableColumn, RowAction, RowActionVariant};
use super::database_server::{fetch_components, mark_component_100, ComponentRow};
use super::edit_component_dialog::EditComponentDialog;
use std::sync::Arc;

#[component]
pub fn ComponentsTable() -> impl IntoView {
    let components_resource = Resource::new(|| (), |_| async { fetch_components().await });
    
    // Modal state
    let edit_dialog_open = RwSignal::new(false);
    let edit_component_id = RwSignal::new(String::new());
    let edit_component_name = RwSignal::new(String::new());
    let edit_component_tipo = RwSignal::new(String::new());
    let edit_tokens_canonicos = RwSignal::new(0);
    let edit_tokens_familia_c = RwSignal::new(0);
    let edit_familias_aplicadas = RwSignal::new(String::new());
    let edit_has_readme = RwSignal::new(false);
    let edit_folder_structure = RwSignal::new(false);
    let edit_status = RwSignal::new(String::new());

    view! {
        <div class="space-y-4">
            <h2 class="text-2xl font-bold mb-4">Components Database</h2>
            <Suspense fallback=|| view! { <p>"Loading components..."</p> }>
                {move || {
                    components_resource.get().map(|result: Result<Vec<ComponentRow>, ServerFnError>| match result {
                        Ok(components) => {
                            let columns = vec![
                                DataTableColumn::new("tipo", "Tipo", |c: &ComponentRow| {
                                    view! { <span class="font-medium">{c.tipo.clone()}</span> }.into_any()
                                })
                                .with_class("font-medium")
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.tipo.cmp(&b.tipo)
                                }),

                                DataTableColumn::new("name", "Nome", |c: &ComponentRow| {
                                    view! { <span class="font-mono text-sm">{c.name.clone()}</span> }.into_any()
                                })
                                .with_class("font-mono")
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.name.cmp(&b.name)
                                }),

                                DataTableColumn::new("tokens", "Tokens", |c: &ComponentRow| {
                                    let percent = c.tokens_canonicos_percent;
                                    let badge_class = if percent == 100 {
                                        "bg-green-100 text-green-800"
                                    } else if percent >= 75 {
                                        "bg-blue-100 text-blue-800"
                                    } else if percent >= 50 {
                                        "bg-yellow-100 text-yellow-800"
                                    } else {
                                        "bg-gray-100 text-gray-800"
                                    };
                                    view! {
                                        <span class=format!("inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {}", badge_class)>
                                            {format!("{}%", percent)}
                                        </span>
                                    }.into_any()
                                })
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.tokens_canonicos_percent.cmp(&b.tokens_canonicos_percent)
                                }),

                                DataTableColumn::new("familias", "Famílias", |c: &ComponentRow| {
                                    let familias = c.familias_aplicadas.clone().unwrap_or_else(|| "".to_string());
                                    if familias.is_empty() {
                                        view! { <span class="text-gray-400">"-"</span> }.into_any()
                                    } else {
                                        let badges: Vec<_> = familias.split(",").map(|f: &str| {
                                            let badge_class = match f.trim() {
                                                "A" => "bg-blue-100 text-blue-800",
                                                "B" => "bg-purple-100 text-purple-800",
                                                "C" => "bg-green-100 text-green-800",
                                                "D" => "bg-yellow-100 text-yellow-800",
                                                "E" => "bg-orange-100 text-orange-800",
                                                "F" => "bg-red-100 text-red-800",
                                                _ => "bg-gray-100 text-gray-800"
                                            };
                                            view! {
                                                <span class=format!("inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium mr-1 {}", badge_class)>
                                                    {f.trim()}
                                                </span>
                                            }
                                        }).collect();
                                        view! { <div class="flex flex-wrap gap-1">{badges}</div> }.into_any()
                                    }
                                })
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.familias_aplicadas.cmp(&b.familias_aplicadas)
                                }),

                                DataTableColumn::new("readme", "README", |c: &ComponentRow| {
                                    view! {
                                        <span class="text-lg">
                                            {if c.has_readme { "✅" } else { "❌" }}
                                        </span>
                                    }.into_any()
                                })
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.has_readme.cmp(&b.has_readme)
                                }),

                                DataTableColumn::new("estrutura", "Estrutura", |c: &ComponentRow| {
                                    view! {
                                        <span class="text-lg">
                                            {if c.folder_structure_correct { "✅" } else { "❌" }}
                                        </span>
                                    }.into_any()
                                })
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.folder_structure_correct.cmp(&b.folder_structure_correct)
                                }),

                                DataTableColumn::new("canonical_type", "Type", |c: &ComponentRow| {
                                    view! { <span class="text-sm">{c.canonical_type.clone()}</span> }.into_any()
                                })
                                .with_class("text-sm")
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.canonical_type.cmp(&b.canonical_type)
                                }),

                                DataTableColumn::new("status", "Status", |c: &ComponentRow| {
                                    let status = c.status.clone();
                                    let badge_class = match status.as_str() {
                                        "stable" => "bg-green-100 text-green-800",
                                        "draft" => "bg-gray-100 text-gray-800",
                                        "review" => "bg-yellow-100 text-yellow-800",
                                        "deprecated" => "bg-red-100 text-red-800",
                                        _ => "bg-gray-100 text-gray-800"
                                    };
                                    view! {
                                        <span class=format!("inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {}", badge_class)>
                                            {status}
                                        </span>
                                    }.into_any()
                                })
                                .with_sort(|a: &ComponentRow, b: &ComponentRow| {
                                    a.status.cmp(&b.status)
                                }),

                                DataTableColumn::actions(move |c: &ComponentRow| {
                                    let component_id = c.id.clone();
                                    let component_name = c.name.clone();
                                    let component_tipo = c.tipo.clone();
                                    let tokens = c.tokens_canonicos_percent;
                                    let familia_c = c.tokens_familia_c_percent;
                                    let familias = c.familias_aplicadas.clone().unwrap_or_default();
                                    let readme = c.has_readme;
                                    let structure = c.folder_structure_correct;
                                    let status = c.status.clone();
                                    
                                    // Clones para cada callback
                                    let name_100 = component_name.clone();
                                    let name_edit = component_name.clone();
                                    
                                    vec![
                                        RowAction {
                                            label: "Marcar como 100%".to_string(),
                                            on_click: Callback::new(move |_| {
                                                let name = name_100.clone();
                                                leptos::logging::log!("Would mark 100% - spawn_local disabled");
                                            }),
                                            variant: RowActionVariant::Default,
                                        },
                                        RowAction {
                                            label: "Editar completo".to_string(),
                                            on_click: Callback::new(move |_| {
                                                edit_component_id.set(component_id.clone());
                                                edit_component_name.set(name_edit.clone());
                                                edit_component_tipo.set(component_tipo.clone());
                                                edit_tokens_canonicos.set(tokens);
                                                edit_tokens_familia_c.set(familia_c);
                                                edit_familias_aplicadas.set(familias.clone());
                                                edit_has_readme.set(readme);
                                                edit_folder_structure.set(structure);
                                                edit_status.set(status.clone());
                                                edit_dialog_open.set(true);
                                            }),
                                            variant: RowActionVariant::Default,
                                        },
                                    ]
                                }),
                            ];

                            view! {
                                <DataTable<ComponentRow>
                                    data=components
                                    columns=columns
                                    get_id=Arc::new(|c: &ComponentRow| c.id.clone())
                                    search_placeholder="Search components..."
                                    selectable=true
                                    filter_fn=Arc::new(|c: &ComponentRow, filter: &str| {
                                        c.tipo.to_lowercase().contains(filter) ||
                                        c.name.to_lowercase().contains(filter) ||
                                        c.familia.to_lowercase().contains(filter) ||
                                        c.status.to_lowercase().contains(filter)
                                    })
                                />
                            }.into_any()
                        },
                        Err(e) => {
                            view! { <p class="text-red-500">"Error: " {e.to_string()}</p> }.into_any()
                        }
                    })
                }}
            </Suspense>
            
            <EditComponentDialog
                open=edit_dialog_open
                component_id=Signal::derive(move || edit_component_id.get())
                component_name=Signal::derive(move || edit_component_name.get())
                component_tipo=Signal::derive(move || edit_component_tipo.get())
                tokens_canonicos=Signal::derive(move || edit_tokens_canonicos.get())
                tokens_familia_c=Signal::derive(move || edit_tokens_familia_c.get())
                familias_aplicadas=Signal::derive(move || edit_familias_aplicadas.get())
                has_readme=Signal::derive(move || edit_has_readme.get())
                folder_structure=Signal::derive(move || edit_folder_structure.get())
                status=Signal::derive(move || edit_status.get())
            />
        </div>
    }
}
