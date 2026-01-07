use leptos::prelude::*;
use rs_design::{DataTable, DataTableColumn};
use super::database_server::{fetch_tokens, TokenRow};
use std::sync::Arc;

#[component]
pub fn TokensTable() -> impl IntoView {
    let tokens_resource = Resource::new(|| (), |_| async { fetch_tokens().await });

    // Filter states
    let selected_scope = RwSignal::new("all".to_string());
    let selected_domain = RwSignal::new("all".to_string());
    let selected_family = RwSignal::new("all".to_string());
    let selected_category = RwSignal::new("all".to_string());
    let selected_name = RwSignal::new("all".to_string());

    // Memo: tokens base
    let tokens = Memo::new(move |_| {
        tokens_resource.get().and_then(|r| r.ok()).unwrap_or_default()
    });

    // Memo: unique options
    let scope_options = Memo::new(move |_| {
        let mut scopes: Vec<String> = tokens.get().iter().map(|t| t.scope.clone()).collect();
        scopes.sort();
        scopes.dedup();
        scopes
    });

    let domain_options = Memo::new(move |_| {
        let mut domains: Vec<String> = tokens.get().iter().map(|t| t.domain.clone()).collect();
        domains.sort();
        domains.dedup();
        domains
    });

    let family_options = Memo::new(move |_| {
        let mut families: Vec<String> = tokens.get().iter()
            .filter_map(|t| t.family_code.clone())
            .collect();
        families.sort();
        families.dedup();
        families
    });

    let category_options = Memo::new(move |_| {
        let mut categories: Vec<String> = tokens.get().iter().map(|t| t.category.clone()).collect();
        categories.sort();
        categories.dedup();
        categories
    });

    let name_options = Memo::new(move |_| {
        let mut names: Vec<String> = tokens.get().iter().map(|t| t.name.clone()).collect();
        names.sort();
        names.dedup();
        names
    });

    // Memo: filtered tokens
    let filtered_tokens = Memo::new(move |_| {
        let scope = selected_scope.get();
        let domain = selected_domain.get();
        let family = selected_family.get();
        let category = selected_category.get();
        let name = selected_name.get();

        tokens.get().iter()
            .filter(|t| {
                (scope == "all" || t.scope == scope)
                && (domain == "all" || t.domain == domain)
                && (family == "all" || t.family_code.as_ref().map(|f| f == &family).unwrap_or(false))
                && (category == "all" || t.category == category)
                && (name == "all" || t.name == name)
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <h2 class="text-2xl font-bold">Tokens Database</h2>
            </div>

            <Suspense fallback=|| view! { <p>"Loading tokens..."</p> }>
                {move || {
                    if tokens.get().is_empty() && tokens_resource.get().is_none() {
                        view! { <p>"Loading..."</p> }.into_any()
                    } else {
                        view! {
                            <div class="space-y-4">
                                <div class="flex items-center gap-2 flex-wrap">
                                    <select
                                        class="h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
                                        on:change=move |ev| selected_scope.set(event_target_value(&ev))
                                    >
                                        <option value="all">"All Scopes"</option>
                                        {move || scope_options.get().into_iter().map(|scope| {
                                            view! { <option value=scope.clone()>{scope.clone()}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>

                                    <select
                                        class="h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
                                        on:change=move |ev| selected_domain.set(event_target_value(&ev))
                                    >
                                        <option value="all">"All Domains"</option>
                                        {move || domain_options.get().into_iter().map(|domain| {
                                            view! { <option value=domain.clone()>{domain.clone()}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>

                                    <select
                                        class="h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
                                        on:change=move |ev| selected_family.set(event_target_value(&ev))
                                    >
                                        <option value="all">"All Families"</option>
                                        {move || family_options.get().into_iter().map(|fam| {
                                            view! { <option value=fam.clone()>{fam.clone()}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>

                                    <select
                                        class="h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
                                        on:change=move |ev| selected_category.set(event_target_value(&ev))
                                    >
                                        <option value="all">"All Categories"</option>
                                        {move || category_options.get().into_iter().map(|cat| {
                                            view! { <option value=cat.clone()>{cat.clone()}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>

                                    <select
                                        class="h-10 rounded-md border border-input bg-background px-3 py-2 text-sm"
                                        on:change=move |ev| selected_name.set(event_target_value(&ev))
                                    >
                                        <option value="all">"All Names"</option>
                                        {move || name_options.get().into_iter().map(|name| {
                                            view! { <option value=name.clone()>{name.clone()}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>

                                    <button
                                        class="h-10 px-4 rounded-md border border-input bg-background text-sm hover:bg-accent"
                                        on:click=move |_| {
                                            selected_scope.set("all".to_string());
                                            selected_domain.set("all".to_string());
                                            selected_family.set("all".to_string());
                                            selected_category.set("all".to_string());
                                            selected_name.set("all".to_string());
                                        }
                                    >
                                        "Reset"
                                    </button>

                                    <span class="text-sm text-muted-foreground ml-2">
                                        {move || format!("{} tokens", filtered_tokens.get().len())}
                                    </span>
                                </div>

                                {
                                    let columns = vec![
                                        DataTableColumn::new("scope", "Scope", |t: &TokenRow| {
                                            let badge_class = match t.scope.as_str() {
                                                "canonical" => "bg-blue-100 text-blue-800",
                                                "contextual" => "bg-purple-100 text-purple-800",
                                                _ => "bg-gray-100 text-gray-800"
                                            };
                                            view! {
                                                <span class=format!("inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {}", badge_class)>
                                                    {t.scope.clone()}
                                                </span>
                                            }.into_any()
                                        })
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.scope.cmp(&b.scope)),

                                        DataTableColumn::new("domain", "Domain", |t: &TokenRow| {
                                            let badge_class = match t.domain.as_str() {
                                                "Core" => "bg-slate-100 text-slate-800",
                                                "Typography" => "bg-indigo-100 text-indigo-800",
                                                "Color" => "bg-pink-100 text-pink-800",
                                                "State" => "bg-amber-100 text-amber-800",
                                                "Motion" => "bg-cyan-100 text-cyan-800",
                                                "Selection" => "bg-purple-100 text-purple-800",
                                                "Forms" => "bg-green-100 text-green-800",
                                                "Overlay" => "bg-blue-100 text-blue-800",
                                                "Navigation" => "bg-orange-100 text-orange-800",
                                                "Feedback" => "bg-red-100 text-red-800",
                                                "Data" => "bg-emerald-100 text-emerald-800",
                                                _ => "bg-gray-100 text-gray-800"
                                            };
                                            view! {
                                                <span class=format!("inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {}", badge_class)>
                                                    {t.domain.clone()}
                                                </span>
                                            }.into_any()
                                        })
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.domain.cmp(&b.domain)),

                                        DataTableColumn::new("family", "Family", |t: &TokenRow| {
                                            if let Some(family) = &t.family_code {
                                                let badge_class = match family.as_str() {
                                                    "A" => "bg-blue-100 text-blue-800",
                                                    "B" => "bg-purple-100 text-purple-800",
                                                    "C" => "bg-green-100 text-green-800",
                                                    "D" => "bg-yellow-100 text-yellow-800",
                                                    "E" => "bg-orange-100 text-orange-800",
                                                    "F" => "bg-red-100 text-red-800",
                                                    _ => "bg-gray-100 text-gray-800"
                                                };
                                                view! {
                                                    <span class=format!("inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {}", badge_class)>
                                                        {family.clone()}
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! { <span class="text-gray-400">"-"</span> }.into_any()
                                            }
                                        })
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.family_code.cmp(&b.family_code)),

                                        DataTableColumn::new("category", "Category", |t: &TokenRow| {
                                            view! { <span class="font-medium">{t.category.clone()}</span> }.into_any()
                                        })
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.category.cmp(&b.category)),

                                        DataTableColumn::new("name", "Name", |t: &TokenRow| {
                                            view! { <code class="font-mono text-sm">{t.name.clone()}</code> }.into_any()
                                        })
                                        .with_class("font-mono")
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.name.cmp(&b.name)),

                                        DataTableColumn::new("value", "Value", |t: &TokenRow| {
                                            view! {
                                                <code class="font-mono text-xs bg-gray-100 px-2 py-1 rounded">
                                                    {t.value.clone()}
                                                </code>
                                            }.into_any()
                                        })
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.value.cmp(&b.value)),

                                        DataTableColumn::new("status", "Status", |t: &TokenRow| {
                                            let status = t.status.clone();
                                            let badge_class = match status.as_str() {
                                                "active" => "bg-green-100 text-green-800",
                                                "deprecated" => "bg-red-100 text-red-800",
                                                _ => "bg-gray-100 text-gray-800"
                                            };
                                            view! {
                                                <span class=format!("inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {}", badge_class)>
                                                    {status}
                                                </span>
                                            }.into_any()
                                        })
                                        .with_sort(|a: &TokenRow, b: &TokenRow| a.status.cmp(&b.status)),
                                    ];

                                    view! {
                                        <DataTable<TokenRow>
                                            data=filtered_tokens.get()
                                            columns=columns
                                            get_id=Arc::new(|t: &TokenRow| t.id.clone())
                                            search_placeholder="Search tokens..."
                                            selectable=false
                                            filter_fn=Arc::new(|t: &TokenRow, filter: &str| {
                                                t.name.to_lowercase().contains(filter) ||
                                                t.category.to_lowercase().contains(filter) ||
                                                t.value.to_lowercase().contains(filter) ||
                                                t.full_name.to_lowercase().contains(filter)
                                            })
                                        />
                                    }
                                }
                            </div>
                        }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
