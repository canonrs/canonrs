use leptos::prelude::*;
use super::language_types::{Language, LanguageContext};

#[component]
pub fn LanguageProvider(
    /// Initial language code (from SSR cookie)
    #[prop(optional)]
    initial_language: Option<String>,
    children: Children,
) -> impl IntoView {
    // Parse initial language
    let lang_code = initial_language
        .as_deref()
        .unwrap_or("en");
    
    let current_language = Language::new(lang_code);
    let current = RwSignal::new(current_language);
    
    let language_context = LanguageContext { current };
    
    // Client: Apply to DOM
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        let lang = current.get();
                        let _ = html.set_attribute("lang", &lang.code);
                        let _ = html.set_attribute("dir", lang.dir_attr());
                    }
                }
            }
        });
    }
    
    provide_context(language_context);
    
    view! { {children()} }
}
