use leptos::prelude::*;
use super::viewport::Viewport;
use super::theme_toolbar::ThemeToolbar;
use crate::application::builder_controller::BuilderController;
use crate::application::wizard_service::{generate_document, WizardConfig};
use crate::ui::theme_engine::token_controls::TokenControls;
use crate::ui::theme_engine::live_preview::LivePreview;

#[component]
pub fn ThemeWorkspace(controller: BuilderController, theme: crate::ui::theme_engine::ThemeState) -> impl IntoView {
    let viewport = RwSignal::new(Viewport::desktop());
    let show_tokens = RwSignal::new(true);
    let root_ref = NodeRef::<leptos::html::Div>::new();
    let theme2 = theme.clone();
    let theme3 = theme.clone();

    Effect::new(move |_| {
        let state = theme.clone();
        let Some(el) = root_ref.get() else { return };
        use leptos::wasm_bindgen::JsCast;
        let html_el: &leptos::web_sys::HtmlElement = el.unchecked_ref();
        let style = html_el.style();
        for item in state.current_css_vars() {
            let _ = style.set_property(&format!("--theme-{}", item.0), &item.1);
        }
        let _ = style.set_property("--preview-radius", &format!("{}rem", state.radius.get()));
    });

    let has_doc = Signal::derive(move || controller.has_document());

    let on_generate = move |_: leptos::ev::MouseEvent| {
        controller.load_document(generate_document(WizardConfig::default()));
    };

    view! {
        <div node_ref=root_ref data-canon-runtime="isolated" id="theme-preview-root" style="display:flex;flex-direction:column;height:100%;min-height:600px;">
            <ThemeToolbar viewport=viewport on_generate=on_generate />
            {move || has_doc.get().then(|| view! {
                <div style="padding:0.35rem 1rem;background:#166534;color:white;font-size:0.75rem;flex-shrink:0;">
                    "✓ Document loaded — switch to 🧱 Builder to edit structure"
                </div>
            })}
            {move || (!has_doc.get()).then(|| view! {
                <div style="padding:0.35rem 1rem;background:var(--theme-surface-bg);color:var(--theme-surface-fg-muted);font-size:0.75rem;flex-shrink:0;border-bottom:1px solid var(--theme-surface-border);">
                    "No document yet — click ⚡ Generate Document to start"
                </div>
            })}
            <div style="flex:1;display:flex;overflow:hidden;">
                <div style=move || if show_tokens.get() { "width:280px;flex-shrink:0;border-right:1px solid var(--theme-surface-border);overflow-y:auto;background:var(--theme-surface-bg);" } else { "display:none;" }>
                    <div style="display:flex;justify-content:space-between;align-items:center;padding:0.5rem 1rem;border-bottom:1px solid var(--theme-surface-border);">
                        <span style="font-size:0.7rem;font-weight:700;text-transform:uppercase;letter-spacing:0.08em;color:var(--theme-surface-fg-muted);">"Tokens"</span>
                        <button on:click=move |_| show_tokens.set(false)
                            style="background:none;border:none;cursor:pointer;color:var(--theme-surface-fg-muted);font-size:0.75rem;">"✕"</button>
                    </div>
                    <TokenControls theme=theme2 />
                </div>
                <button
                    style=move || if !show_tokens.get() { "writing-mode:vertical-rl;padding:0.5rem 0.25rem;border:none;border-right:1px solid var(--theme-surface-border);cursor:pointer;background:var(--theme-surface-bg);color:var(--theme-surface-fg-muted);font-size:0.7rem;font-weight:700;text-transform:uppercase;letter-spacing:0.05em;" } else { "display:none;" }
                    on:click=move |_| show_tokens.set(true)
                >"Tokens"</button>
                <div style="flex:1;overflow:auto;background:#e5e7eb;display:flex;justify-content:center;padding:2rem;">
                    <div style=move || format!(
                        "width:{}px;min-height:{}px;background:white;box-shadow:0 4px 24px rgba(0,0,0,0.12);border-radius:8px;overflow:hidden;transition:width 0.3s;",
                        viewport.get().width, viewport.get().height
                    )>
                        <LivePreview theme=theme3 />
                    </div>
                </div>
            </div>
        </div>
    }
}
