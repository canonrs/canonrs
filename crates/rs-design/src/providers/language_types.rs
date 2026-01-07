use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language {
    pub code: String,      // "en", "pt", "es"
    pub name: String,      // "English", "Português", "Español"
    pub direction: TextDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextDirection {
    Ltr,  // Left-to-Right (English, Portuguese, Spanish)
    Rtl,  // Right-to-Left (Arabic, Hebrew)
}

impl Language {
    pub fn new(code: &str) -> Self {
        match code {
            "pt" | "pt-BR" => Self {
                code: "pt".to_string(),
                name: "Português".to_string(),
                direction: TextDirection::Ltr,
            },
            "es" => Self {
                code: "es".to_string(),
                name: "Español".to_string(),
                direction: TextDirection::Ltr,
            },
            "ar" => Self {
                code: "ar".to_string(),
                name: "العربية".to_string(),
                direction: TextDirection::Rtl,
            },
            _ => Self {
                code: "en".to_string(),
                name: "English".to_string(),
                direction: TextDirection::Ltr,
            },
        }
    }
    
    pub fn dir_attr(&self) -> &'static str {
        match self.direction {
            TextDirection::Ltr => "ltr",
            TextDirection::Rtl => "rtl",
        }
    }
}

#[derive(Clone, Copy)]
pub struct LanguageContext {
    pub current: RwSignal<Language>,
}

pub fn use_language() -> LanguageContext {
    use_context::<LanguageContext>()
        .expect("LanguageContext not found. Make sure LanguageProvider is in the component tree.")
}
