use leptos::prelude::*;
use crate::application::document_service;
use crate::application::document_service::SaveError;
use super::types::{CanvasMode, Node, ActiveLayout, NodeKind};
use rs_canonrs::domain::CanonDocument;
use super::application::{build_tree, flatten_tree};
use super::codegen::export_document_to_rs;
use canonrs_ui::ui::toolbar::toolbar_ui::{Toolbar, ToolbarSeparator};
use super::state::builder_engine::BuilderEngine;
use canonrs_ui::ui::button::button_ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn CanvasToolbar(
    canvas_mode: RwSignal<CanvasMode>,
    slots: RwSignal<Vec<Node>>,
    tree: RwSignal<Vec<Node>>,
    active_layout: RwSignal<ActiveLayout>,
    engine: RwSignal<BuilderEngine>,
    is_dirty: RwSignal<bool>,
) -> impl IntoView {
    let save_status: RwSignal<Option<String>> = RwSignal::new(None);
    let doc_name: RwSignal<String> = RwSignal::new("Untitled Layout".to_string());
    let show_load: RwSignal<bool> = RwSignal::new(false);
    let doc_list: RwSignal<Vec<(String, String, String)>> = RwSignal::new(vec![]);
    let doc_version: RwSignal<i64> = RwSignal::new(1);

    let on_undo = move |_: leptos::ev::MouseEvent| {
        engine.update(|e| { e.undo(); });
        let flat = engine.get_untracked().sync_flat();
        tree.set(flat);
        is_dirty.set(true);
    };

    let on_redo = move |_: leptos::ev::MouseEvent| {
        engine.update(|e| { e.redo(); });
        let flat = engine.get_untracked().sync_flat();
        tree.set(flat);
        is_dirty.set(true);
    };
    let doc_id: RwSignal<String> = RwSignal::new(uuid::Uuid::new_v4().to_string());

    let on_view_published = move |_: leptos::ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            let id = doc_id.get();
            if let Some(window) = web_sys::window() {
                let url = format!("http://localhost:8113/app/{}", id);
                let _ = window.open_with_url_and_target(&url, "_blank");
            }
        }
    };

    let on_publish = move |_: leptos::ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            let id = doc_id.get();
            leptos::logging::log!("[PUBLISH] doc_id={}", id);
            if id.is_empty() {
                save_status.set(Some("Save first".to_string()));
                return;
            }
            save_status.set(Some("Publishing...".to_string()));
            leptos::task::spawn_local(async move {
                match document_service::publish_document(id.clone()).await {
                    Ok(version) => {
                        let _ = document_service::invalidate_cache(id).await;
                        save_status.set(Some(format!("Published v{}", version)));
                    }
                    Err(e) => save_status.set(Some(format!("Publish error: {}", e))),
                }
                leptos::task::spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(3000).await;
                    save_status.set(None);
                });
            });
        }
    };

    let on_export = move |_: leptos::ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            let all_nodes = {
                let mut combined = slots.get();
                combined.extend(tree.get());
                combined
            };
            let doc = CanonDocument {
                id: uuid::Uuid::new_v4(),
                layout: format!("{:?}", active_layout.get()),
                version: 1,
                nodes: build_tree(&all_nodes),
            };
            let code = export_document_to_rs(&doc);
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    use leptos::wasm_bindgen::JsCast;
                    let blob_parts = js_sys::Array::new();
                    blob_parts.push(&leptos::wasm_bindgen::JsValue::from_str(&code));
                    if let Ok(blob) = web_sys::Blob::new_with_str_sequence(&blob_parts) {
                        if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                            if let Ok(a) = document.create_element("a") {
                                let _ = a.set_attribute("href", &url);
                                let _ = a.set_attribute("download", "exported_layout.rs");
                                if let Some(body) = document.body() {
                                    let _ = body.append_child(&a);
                                    if let Some(el) = a.dyn_ref::<web_sys::HtmlElement>() {
                                        el.click();
                                    }
                                    let _ = body.remove_child(&a);
                                }
                                let _ = web_sys::Url::revoke_object_url(&url);
                            }
                        }
                    }
                }
            }
        }
    };

    let on_save = move |_: leptos::ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            let all_nodes = {
                let mut combined = slots.get();
                combined.extend(tree.get());
                combined
            };
            let doc = CanonDocument {
                id: uuid::Uuid::parse_str(&doc_id.get()).unwrap_or_else(|_| uuid::Uuid::new_v4()),
                layout: format!("{:?}", active_layout.get()),
                version: doc_version.get() as u32,
                nodes: build_tree(&all_nodes),
            };
            if let Ok(payload_json) = doc.to_json() {
                let name = doc_name.get();
                let id_str = doc.id.to_string();
                let layout_str = doc.layout.clone();
                let version = doc_version.get();
                save_status.set(Some("Saving...".to_string()));

                leptos::task::spawn_local(async move {
                    let body = serde_json::json!({
                        "id": id_str,
                        "name": name,
                        "layout": layout_str,
                        "version": version,
                        "payload_json": payload_json,
                    });
                    match document_service::save_document(id_str.clone(), name, layout_str, version, payload_json).await {
                        Ok(new_version) => {
                            doc_version.set(new_version);
                            save_status.set(Some(format!("Saved v{}", new_version)));
                        }
                        Err(SaveError::Conflict { current_version, sent_version }) => {
                            save_status.set(Some(format!(
                                "Conflict: server=v{} local=v{}", current_version, sent_version
                            )));
                        }
                        Err(SaveError::Other(e)) => save_status.set(Some(format!("Error: {}", e))),
                    }
                    leptos::task::spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(3000).await;
                        save_status.set(None);
                    });
                });
            }
        }
    };

    let on_load_list = move |_: leptos::ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            leptos::task::spawn_local(async move {
                match document_service::list_documents().await {
                    Ok(list) => {
                        leptos::logging::log!("[LOAD] list ok len={}", list.len());
                        doc_list.set(list);
                        show_load.set(true);
                    }
                    Err(e) => {
                        leptos::logging::log!("[LOAD] error: {}", e);
                        save_status.set(Some(format!("Error: {}", e)));
                    }
                }
            });
        }
    };

    let on_load_doc = move |id: String, version: i64| {
        #[cfg(target_arch = "wasm32")]
        {
            let id_clone = id.clone();
            leptos::task::spawn_local(async move {
                match document_service::get_document(id_clone.clone()).await {
                    Ok((payload_json, layout_str, v)) => {
                        if let Ok(doc) = CanonDocument::from_json(&payload_json) {
                            let flat = flatten_tree(&doc.nodes);
                            let (new_slots, new_tree): (Vec<_>, Vec<_>) = flat.into_iter()
                                .partition(|n| matches!(n.kind, NodeKind::Slot { .. }));
                            slots.set(new_slots);
                            tree.set(new_tree);
                            if let Some(layout) = parse_layout(&layout_str) {
                                active_layout.set(layout);
                            }
                            doc_id.set(id_clone);
                            doc_version.set(v);
                            show_load.set(false);
                            save_status.set(Some(format!("Loaded v{}", v)));
                        }
                    }
                    Err(e) => save_status.set(Some(format!("Error: {}", e))),
                }
            });
        }
        let _ = version;
    };

    let can_undo = Signal::derive(move || engine.get().can_undo());
    let can_redo = Signal::derive(move || engine.get().can_redo());

    view! {
        <div style="display: flex; flex-direction: column; gap: 0.25rem; position: relative;">
            <Toolbar aria_label="Canvas toolbar">
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm
                    on:click=move |_| canvas_mode.set(CanvasMode::Builder)>
                    "Builder"
                </Button>
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm
                    on:click=move |_| canvas_mode.set(CanvasMode::Preview)>
                    "Preview"
                </Button>
                <ToolbarSeparator />
                <input
                    type="text"
                    prop:value=move || doc_name.get()
                    on:input=move |ev| doc_name.set(event_target_value(&ev))
                    placeholder="Layout name..."
                    style="padding: 0.2rem 0.5rem; font-size: 0.75rem; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); background: var(--theme-surface-bg); color: var(--theme-surface-fg); width: 140px;"
                />
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm on:click=on_save>
                    "Save"
                </Button>
            </Toolbar>

            <Toolbar aria_label="Actions toolbar">
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=on_load_list>
                    "Open"
                </Button>
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=on_export>
                    "Export .rs"
                </Button>
                <ToolbarSeparator />
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm on:click=on_publish>
                    "Publish"
                </Button>
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Sm on:click=on_view_published>
                    "View"
                </Button>
                <ToolbarSeparator />
                <button
                    on:click=on_undo
                    style="padding: 0.2rem 0.5rem; font-size: 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); cursor: pointer; background: var(--theme-surface-bg); color: var(--theme-surface-fg);"
                >
                    "Undo"
                </button>
                <button
                    on:click=on_redo
                    style="padding: 0.2rem 0.5rem; font-size: 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-surface-border); cursor: pointer; background: var(--theme-surface-bg); color: var(--theme-surface-fg);"
                >
                    "Redo"
                </button>
                {move || is_dirty.get().then(|| view! {
                    <span style="font-size: 0.7rem; color: var(--theme-action-warning-fg, orange);">
                        "● unsaved"
                    </span>
                })}
                {move || save_status.get().map(|s| view! {
                    <span style="font-size: 0.7rem; color: var(--theme-surface-fg-muted);">{s}</span>
                })}
            </Toolbar>

            // Painel de lista de documentos
            {move || if show_load.get() {
                let list = doc_list.get();
                view! {
                    <div style="position: absolute; top: 4rem; z-index: 100; background: var(--theme-surface-bg); border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); padding: 0.5rem; min-width: 280px; box-shadow: var(--shadow-lg);">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                            <span style="font-size: 0.75rem; font-weight: 600;">"Saved Layouts"</span>
                            <button on:click=move |_| show_load.set(false)
                                style="background: none; border: none; cursor: pointer; color: var(--theme-surface-fg-muted);">
                                "x"
                            </button>
                        </div>
                        {list.into_iter().map(|(id, name, updated_at)| {
                            let id_clone = id.clone();
                            let load = on_load_doc.clone();
                            view! {
                                <div
                                    on:click=move |_| load(id_clone.clone(), 0)
                                    style="padding: 0.4rem 0.5rem; cursor: pointer; border-radius: var(--radius-sm); font-size: 0.8rem; display: flex; justify-content: space-between;"
                                >
                                    <span>{name}</span>
                                    <span style="color: var(--theme-surface-fg-muted); font-size: 0.7rem;">{updated_at}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any()
            } else {
                view! { <div /> }.into_any()
            }}
        </div>
    }
}

fn parse_layout(s: &str) -> Option<ActiveLayout> {
    match s {
        "Dashboard" => Some(ActiveLayout::Dashboard),
        "Marketing" => Some(ActiveLayout::Marketing),
        "Fullscreen" => Some(ActiveLayout::Fullscreen),
        "SplitView" => Some(ActiveLayout::SplitView),
        "Wizard" => Some(ActiveLayout::Wizard),
        "Section" => Some(ActiveLayout::Section),
        "PageSingle" => Some(ActiveLayout::PageSingle),
        "PageWithSidebar" => Some(ActiveLayout::PageWithSidebar),
        "PageWithAside" => Some(ActiveLayout::PageWithAside),
        "PageSidebarAndAside" => Some(ActiveLayout::PageSidebarAndAside),
        _ => None,
    }
}

