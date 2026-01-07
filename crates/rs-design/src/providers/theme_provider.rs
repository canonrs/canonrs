use leptos::prelude::*;
use super::theme_types::{ThemeMode, ThemeContext};
use crate::themes::ThemeRegistry;

#[component]
pub fn ThemeProvider(
    /// Initial theme mode (from SSR cookie)
    #[prop(optional)]
    initial_mode: Option<String>,
    /// Initial theme preset (from SSR cookie)
    #[prop(optional)]
    initial_preset: Option<String>,
    children: Children,
) -> impl IntoView {
    // Parse initial values
    let parsed_mode = initial_mode
        .as_deref()
        .and_then(|m| match m {
            "dark" => Some(ThemeMode::Dark),
            "light" => Some(ThemeMode::Light),
            _ => None,
        })
        .unwrap_or(ThemeMode::System);
    
    let parsed_preset = initial_preset
        .as_deref()
        .filter(|p| ThemeRegistry::get_preset(p).is_some())
        .unwrap_or_else(|| ThemeRegistry::default_preset())
        .to_string();
    
    // Estado reativo
    let mode = RwSignal::new(parsed_mode);
    let preset = RwSignal::new(parsed_preset);
    
    let theme_context = ThemeContext { mode, preset };
    
    // Cliente: aplicar ao DOM
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        
        // Aplicar data-theme e class ao HTML
        Effect::new(move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        // Aplicar data-theme (preset)
                        let _ = html.set_attribute("data-theme", &preset.get());
                        
                        // Resolver mode para class (dark/light)
                        let resolved_mode = match mode.get() {
                            ThemeMode::Dark => "dark",
                            ThemeMode::Light => "light",
                            ThemeMode::System => {
                                if window.match_media("(prefers-color-scheme: dark)")
                                    .ok()
                                    .flatten()
                                    .map(|mq| mq.matches())
                                    .unwrap_or(false)
                                {
                                    "dark"
                                } else {
                                    "light"
                                }
                            }
                        };
                        
                        let class_list = html.class_list();
                        let _ = class_list.remove_1("light");
                        let _ = class_list.remove_1("dark");
                        let _ = class_list.add_1(resolved_mode);
                    }
                }
            }
        });
    }
    
    provide_context(theme_context);
    
    view! { {children()} }
}
