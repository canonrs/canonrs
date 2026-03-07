use leptos::prelude::*;

#[component]
pub fn LanguageProvider(lang_code: String, children: Children) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        use leptos::leptos_dom::helpers::document;
        
        Effect::new(move |_| {
            if let Some(html) = document().document_element() {
                let _ = html.set_attribute("lang", &lang_code);
            }
        });
    }
    
    children()
}
