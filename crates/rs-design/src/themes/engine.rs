use leptos::prelude::*;
use leptos::logging::log;
use crate::providers::{use_theme, ThemeMode};

/// Engine que observa contexto e aplica efeitos visuais
#[component]
pub fn ThemeEngine() -> impl IntoView {
    let theme = use_theme();
    
    // Effect 1: Aplicar mode (dark/light class)
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let current_mode = theme.mode.get();
            
            let resolved_mode = match current_mode {
                ThemeMode::System => {
                    if let Ok(media) = window.match_media("(prefers-color-scheme: dark)") {
                        if let Some(matches) = media {
                            if matches.matches() {
                                ThemeMode::Dark
                            } else {
                                ThemeMode::Light
                            }
                        } else {
                            ThemeMode::Light
                        }
                    } else {
                        ThemeMode::Light
                    }
                }
                other => other,
            };
            
            if let Some(doc) = window.document() {
                if let Some(html) = doc.document_element() {
                    let class_list = html.class_list();
                    match resolved_mode {
                        ThemeMode::Light => {
                            let _ = class_list.remove_1("dark");
                            log!("Mode applied: light");
                        }
                        ThemeMode::Dark => {
                            let _ = class_list.add_1("dark");
                            log!("Mode applied: dark");
                        }
                        ThemeMode::System => {}
                    }
                }
            }
        }
    });
    
    // Effect 2: Aplicar preset (data-theme attribute)
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            let preset = theme.preset.get();
            
            if let Some(doc) = window.document() {
                if let Some(html) = doc.document_element() {
                    let _ = html.set_attribute("data-theme", &preset);
                    log!("Preset applied: {}", preset);
                }
            }
        }
    });
    
    // Retorna fragmento vazio (não renderiza nada visualmente)
    ().into_view()
}
