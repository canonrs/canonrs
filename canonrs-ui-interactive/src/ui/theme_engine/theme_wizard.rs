use leptos::prelude::*;
use super::ThemeState;
use super::token_controls::TokenControls;
use super::live_preview::LivePreview;
use crate::application::builder_controller::BuilderController;
use crate::application::wizard_service::{generate_document, WizardConfig};

#[component]
pub fn ThemeWizard(controller: BuilderController) -> impl IntoView {
    use crate::infra::app_state::{global_theme, app_owner};
    let theme = global_theme();
    let generated = RwSignal::new(false);

    let on_generate = move |_: leptos::ev::MouseEvent| {
        let doc = generate_document(WizardConfig::default());
        controller.load_document(doc);
        generated.set(true);
    };

    view! {
        <div style="display:flex;flex-direction:column;height:100%;min-height:500px;">
            <div style="display:flex;align-items:center;justify-content:space-between;padding:0.75rem 1rem;border-bottom:1px solid var(--theme-surface-border);background:var(--theme-surface-bg);">
                <span style="font-size:0.75rem;font-weight:700;text-transform:uppercase;letter-spacing:0.08em;color:var(--theme-surface-fg-muted);">
                    "🎨 Theme Engine — Contrato Visual"
                </span>
                <button on:click=on_generate
                    style="padding:0.3rem 0.8rem;border-radius:4px;border:none;cursor:pointer;font-size:0.8rem;font-weight:600;background:var(--theme-primary-bg);color:var(--theme-primary-fg);">
                    "⚡ Generate Document"
                </button>
            </div>
            {move || generated.get().then(|| view! {
                <div style="padding:0.4rem 1rem;background:#166534;color:white;font-size:0.75rem;">
                    "✓ Document generated — switch to Builder to see it"
                </div>
            })}
            <div style="display:flex;flex:1;overflow:hidden;">
                <div style="width:300px;flex-shrink:0;border-right:1px solid var(--theme-surface-border);overflow-y:auto;background:var(--theme-surface-bg);">
                    <TokenControls theme=theme.clone() />
                </div>
                <div style="flex:1;overflow:auto;padding:1.5rem;background:var(--theme-page-bg);">
                    <LivePreview theme=theme.clone() />
                </div>
            </div>
        </div>
    }
}
