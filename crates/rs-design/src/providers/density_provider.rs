use leptos::prelude::*;
use super::density_types::{DensityMode, DensityContext};

#[component]
pub fn DensityProvider(
    /// Initial density mode (from SSR or user preference)
    #[prop(optional)]
    initial_mode: Option<String>,
    children: Children,
) -> impl IntoView {
    // Parse initial value
    let parsed_mode = initial_mode
        .as_deref()
        .map(DensityMode::from_str)
        .unwrap_or_default();
    
    let mode = RwSignal::new(parsed_mode);
    let density_context = DensityContext { mode };
    
    // Client: Apply to DOM
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let _ = html.set_attribute("data-density", mode.get().as_str());
                    }
                }
            }
        });
    }
    
    provide_context(density_context);
    
    view! { {children()} }
}
