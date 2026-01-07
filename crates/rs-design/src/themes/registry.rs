#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemePreset {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

pub struct ThemeRegistry;

impl ThemeRegistry {
    pub fn available_presets() -> Vec<ThemePreset> {
        vec![
            ThemePreset {
                id: "clean-slate",
                label: "Clean Slate",
                description: "Minimal, clean design with neutral colors",
            },
            ThemePreset {
                id: "amber-minimal",
                label: "Amber Minimal",
                description: "Warm amber accents with minimalist approach",
            },
        ]
    }
    
    pub fn default_preset() -> &'static str {
        "clean-slate"
    }
    
    pub fn get_preset(id: &str) -> Option<ThemePreset> {
        Self::available_presets()
            .into_iter()
            .find(|p| p.id == id)
    }
}
