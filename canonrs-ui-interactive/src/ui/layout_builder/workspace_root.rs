use leptos::prelude::*;
use super::builder_workspace::BuilderWorkspace;
use crate::application::builder_controller::BuilderController;
use crate::ui::theme_workspace::theme_workspace::ThemeWorkspace;
use crate::ui::theme_workspace::viewport::Viewport;
use crate::infra::app_state::*;
use crate::infra::persistence::{load_theme_into, persist_theme};
use crate::infra::export::{export_css, export_json, export_builder, import_builder};

fn trigger_download(filename: &str, content: &str) {
    use leptos::wasm_bindgen::JsValue;
    use web_sys::{Blob, Url, HtmlAnchorElement};
    use leptos::wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str(content));
    let Ok(blob) = Blob::new_with_str_sequence(&arr) else { return };
    let Ok(url) = Url::create_object_url_with_blob(&blob) else { return };
    let Ok(el) = document.create_element("a") else { return };
    let anchor: HtmlAnchorElement = el.unchecked_into();
    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();
    let _ = Url::revoke_object_url(&url);
}

fn vp_btn(label: &'static str, vp: Viewport, current: RwSignal<Viewport>) -> impl IntoView {
    let is_active = move || current.get() == vp;
    view! {
        <button
            on:click=move |_| current.set(vp)
            style=move || format!(
                "padding:0.25rem 0.6rem;border-radius:4px;border:1px solid var(--theme-surface-border);cursor:pointer;font-size:0.75rem;background:{};color:{};",
                if is_active() {"var(--theme-primary-bg)"} else {"transparent"},
                if is_active() {"var(--theme-primary-fg)"} else {"var(--theme-surface-fg)"}
            )
        >{label}</button>
    }
}

#[component]
pub fn LayoutBuilderInteractive() -> impl IntoView {
    let mode = global_workspace_mode();
    let viewport = RwSignal::new(Viewport::desktop());
    let theme = global_theme();
    load_theme_into(&theme);
    persist_theme(theme.clone());
    // Bootstrap engine com layout inicial
    crate::infra::app_state::bootstrap_engine_with_layout(&global_active_layout().get_untracked());
    let engine = global_engine();
    let tree = global_tree();
    let selected_id = global_selected();
    let is_dirty = global_dirty();
    let active_layout = global_active_layout();
    let slots = global_slots();
    let drag_ctx = global_drag_ctx();
    let canvas_mode = global_canvas_mode();
    let drag_visual = global_drag_visual();

    let theme_css  = theme.clone();
    let theme_json = theme.clone();

    let on_export_css = move |_: leptos::ev::MouseEvent| {
        trigger_download("canonrs.custom.css", &export_css(&theme_css));
    };
    let on_export_json = move |_: leptos::ev::MouseEvent| {
        trigger_download("theme.json", &export_json(&theme_json));
    };
    let on_export_builder = move |_: leptos::ev::MouseEvent| {
        let t = tree.get();
        let layout = format!("{:?}", active_layout.get());
        trigger_download("builder.json", &export_builder(&t, &layout));
    };
    let on_load_builder = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        use leptos::wasm_bindgen::closure::Closure;
        use leptos::wasm_bindgen::JsValue;
        let input = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
        let Some(file) = input.files().and_then(|f| f.get(0)) else { return };
        let reader = web_sys::FileReader::new().unwrap();
        let reader2 = reader.clone();
        let tree2 = tree;
        let closure = Closure::once(move |_: JsValue| {
            let result = reader2.result().unwrap();
            if let Some(text) = result.as_string() {
                if let Some(nodes) = import_builder(&text) {
                    tree2.set(nodes);
                }
            }
        });
        reader.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        let _ = reader.read_as_text(&file);
    };

    let controller = BuilderController::new(engine, tree, selected_id, is_dirty);

    view! {
        <div style="display:flex;flex-direction:column;height:100%;min-height:600px;border:1px solid var(--theme-surface-border);border-radius:var(--radius-md);overflow:hidden;">
            // BARRA GLOBAL
            <div style="display:flex;align-items:center;gap:0.5rem;padding:0.4rem 1rem;border-bottom:1px solid var(--theme-surface-border);background:var(--theme-surface-bg);flex-shrink:0;">
                // Esquerda — modo
                <button
                    on:click=move |_| mode.set(WorkspaceMode::Builder)
                    style=move || format!("padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:{};color:{};",
                        if mode.get()==WorkspaceMode::Builder {"var(--theme-primary-bg)"} else {"transparent"},
                        if mode.get()==WorkspaceMode::Builder {"var(--theme-primary-fg)"} else {"var(--theme-surface-fg)"}
                    )>"🧱 Builder"</button>
                <button
                    on:click=move |_| mode.set(WorkspaceMode::Theme)
                    style=move || format!("padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:{};color:{};",
                        if mode.get()==WorkspaceMode::Theme {"var(--theme-primary-bg)"} else {"transparent"},
                        if mode.get()==WorkspaceMode::Theme {"var(--theme-primary-fg)"} else {"var(--theme-surface-fg)"}
                    )>"🎨 Theme Engine"</button>

                // Centro — viewport
                <div style="flex:1;display:flex;justify-content:center;gap:0.4rem;">
                    {vp_btn("🖥 Desktop", Viewport::desktop(), viewport)}
                    {vp_btn("📱 Tablet",  Viewport::tablet(),  viewport)}
                    {vp_btn("📲 Mobile",  Viewport::mobile(),  viewport)}
                </div>

                // Direita — export
                <button on:click=on_export_css
                    style="padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:transparent;color:var(--theme-surface-fg);"
                >"📋 CSS"</button>
                <button on:click=on_export_json
                    style="padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:transparent;color:var(--theme-surface-fg);"
                >"📦 JSON"</button>
                <button on:click=on_export_builder
                    style="padding:0.25rem 0.75rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:transparent;color:var(--theme-surface-fg);"
                >"📐 Layout"</button>
                <label style="padding:0.25rem 0.75rem;border-radius:4px;cursor:pointer;font-size:0.8rem;font-weight:600;background:transparent;color:var(--theme-surface-fg);">
                    "📂 Load"
                    <input type="file" accept=".json" style="display:none;" on:change=on_load_builder />
                </label>
            </div>

            <div style=move || if mode.get()==WorkspaceMode::Theme { "flex:1;overflow:hidden;" } else { "display:none;" }>
                <ThemeWorkspace controller=controller theme=theme.clone() viewport=viewport />
            </div>
            <div style=move || if mode.get()==WorkspaceMode::Builder { "flex:1;overflow:hidden;" } else { "display:none;" }>
                <BuilderWorkspace
                    controller=controller
                    active_layout=active_layout
                    slots=slots
                    tree=tree
                    engine=engine
                    drag_ctx=drag_ctx
                    canvas_mode=canvas_mode
                    drag_visual=drag_visual
                    viewport=viewport
                />
            </div>
        </div>
    }
}
