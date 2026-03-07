use leptos::prelude::*;
use super::theme_types::{ThemeContext, ThemeMode};

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // Sempre cria signals (funciona em SSR)
    let mode = create_rw_signal(ThemeMode::Dark);
    let preset = create_rw_signal("canonrs".to_string());
    
    provide_context(ThemeContext { mode, preset });
    
    // Effects só rodam no cliente
    #[cfg(feature = "hydrate")]
    {
        use leptos::leptos_dom::helpers::document;
        
        Effect::new(move |_| {
            if let Some(html) = document().document_element() {
                match mode.get() {
                    ThemeMode::Dark => {
                        let _ = html.class_list().add_1("dark");
                    }
                    ThemeMode::Light | ThemeMode::System => {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }
        });
    }
    
    children()
}
