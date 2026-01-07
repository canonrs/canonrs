use leptos::prelude::*;
use crate::providers::use_theme;
use crate::ui::select::Select;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}

#[component]
pub fn DensityPicker(
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let theme = use_theme();
    let density = RwSignal::new("comfortable".to_string());
    
    view! {
        <Select
            value=density
            on_change=Callback::new(move |value: String| {
                // Aplicar data-density no <html>
                #[cfg(target_arch = "wasm32")]
                if let Some(window) = web_sys::window() {
                    if let Some(doc) = window.document() {
                        if let Some(html) = doc.document_element() {
                            let _ = html.set_attribute("data-density", &value);
                        }
                    }
                }
                density.set(value);
            })
            class=class
        >
            <option value="compact">"Compact"</option>
            <option value="comfortable">"Comfortable"</option>
            <option value="spacious">"Spacious"</option>
        </Select>
    }
}
