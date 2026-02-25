use leptos::prelude::*;
use leptos::web_sys;
use super::ThemeState;

#[component]
pub fn LivePreview(theme: ThemeState) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();
    let t = theme.clone();

    Effect::new(move |_| {
        let _ = t.active.get();
        let _ = t.light.get();
        let _ = t.dark.get();
        let _ = t.radius.get();
        let Some(el) = node_ref.get() else { return };
        use leptos::wasm_bindgen::JsCast;
        let html_el: &web_sys::HtmlElement = el.unchecked_ref();
        let style = html_el.style();
        for item in t.current_css_vars() {
            let _ = style.set_property(&format!("--theme-{}", item.0), &item.1);
        }
        let _ = style.set_property("--preview-radius", &format!("{}rem", t.radius.get()));
    });

    view! {
        <div node_ref=node_ref style="height:100%;min-height:400px;background:var(--theme-surface-bg);color:var(--theme-surface-fg);padding:1rem;font-size:14px;">
            <div style="font-size:0.65rem;font-weight:700;text-transform:uppercase;letter-spacing:0.1em;opacity:0.4;margin-bottom:1rem;">"LIVE PREVIEW"</div>
            <div style="margin-bottom:1.5rem;display:flex;gap:0.5rem;">
                <button style="background:var(--theme-action-primary-bg);color:var(--theme-action-primary-fg);padding:0.5rem 1rem;border-radius:var(--preview-radius,8px);border:none;cursor:pointer;font-weight:600;">"Primary"</button>
                <button style="background:var(--theme-action-secondary-bg);color:var(--theme-action-secondary-fg);padding:0.5rem 1rem;border-radius:var(--preview-radius,8px);border:none;cursor:pointer;font-weight:600;">"Secondary"</button>
                <button style="background:var(--theme-action-accent-bg);color:var(--theme-action-accent-fg);padding:0.5rem 1rem;border-radius:var(--preview-radius,8px);border:none;cursor:pointer;font-weight:600;">"Accent"</button>
            </div>
            <div style="background:var(--theme-surface-elevated);border:1px solid var(--theme-surface-border);border-radius:var(--preview-radius,8px);padding:1rem;margin-bottom:1rem;">
                <div style="font-weight:600;margin-bottom:0.4rem;">"Card Component"</div>
                <div style="color:var(--theme-surface-fg-muted);font-size:0.85em;">"Tokens como contrato arquitetural."</div>
            </div>
            <div style="display:flex;gap:0.5rem;margin-bottom:1rem;">
                <span style="background:var(--theme-state-success-bg);color:var(--theme-state-success-fg);padding:0.25rem 0.6rem;border-radius:var(--preview-radius,8px);font-size:0.8em;font-weight:600;">"✓ Success"</span>
                <span style="background:var(--theme-state-warning-bg);color:var(--theme-state-warning-fg);padding:0.25rem 0.6rem;border-radius:var(--preview-radius,8px);font-size:0.8em;font-weight:600;">"⚠ Warning"</span>
                <span style="background:var(--theme-state-error-bg);color:var(--theme-state-error-fg);padding:0.25rem 0.6rem;border-radius:var(--preview-radius,8px);font-size:0.8em;font-weight:600;">"✕ Error"</span>
            </div>
            <div style="display:flex;border:1px solid var(--theme-surface-border);border-radius:var(--preview-radius,8px);overflow:hidden;height:80px;">
                <div style="width:60px;background:var(--theme-sidebar-bg);border-right:1px solid var(--theme-sidebar-border);padding:0.5rem;">
                    <div style="width:24px;height:4px;background:var(--theme-sidebar-primary-bg);border-radius:2px;margin-bottom:4px;"></div>
                    <div style="width:24px;height:4px;background:var(--theme-sidebar-fg);opacity:0.3;border-radius:2px;"></div>
                </div>
                <div style="flex:1;padding:0.5rem;font-size:0.7rem;color:var(--theme-surface-fg-muted);">"Main content"</div>
            </div>
        </div>
    }
}
