//! @canon-level: strict
//! ComponentMeta registry — mapeamento nome -> meta
//! HasMeta via trait não funciona em Leptos components (são funções).
//! Usar component_meta("Dialog") para lookup em runtime (RAG/Decision Engine).

use crate::meta::{ComponentMeta, all_meta};

/// Lookup de ComponentMeta por nome de componente.
/// Usado por Decision Engine e RAG layer.
pub fn component_meta(name: &str) -> Option<&'static ComponentMeta> {
    all_meta().into_iter().find(|m| m.name == name).map(|&ref m| m)
}

/// Retorna família de um componente por nome.
pub fn component_family(name: &str) -> Option<&'static str> {
    component_meta(name).map(|m| m.family.as_str())
}

/// Retorna capabilities de um componente por nome.
pub fn component_capabilities(name: &str) -> Option<&'static [crate::meta::Capability]> {
    component_meta(name).map(|m| m.capabilities)
}

/// Retorna required_parts de um componente por nome.
pub fn component_required_parts(name: &str) -> Option<&'static [&'static str]> {
    component_meta(name).map(|m| m.required_parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_meta() {
        let meta = component_meta("Dialog").unwrap();
        assert_eq!(meta.family.as_str(), "overlay");
        assert!(meta.required_parts.contains(&"DialogTitle"));
    }

    #[test]
    fn test_button_meta() {
        let meta = component_meta("Button").unwrap();
        assert!(!meta.composable);
        assert!(!meta.capabilities.is_empty());
    }
}
