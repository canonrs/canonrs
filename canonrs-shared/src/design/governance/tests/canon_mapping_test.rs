#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::component_resolver::resolve_canonical_component;

    #[test]
    fn every_ui_has_family() {
        let registry = component_registry();
        let mut missing = Vec::new();
        
        for &ui_snake in DISCOVERED_UI_COMPONENTS {
            // Use SEMANTIC RESOLVER (not naive conversion)
            let canonical = resolve_canonical_component(ui_snake);
            
            if !registry.contains_key(canonical) {
                missing.push(format!("{} → {}", ui_snake, canonical));
            }
        }
        
        assert!(
            missing.is_empty(),
            "UI components without family:\n{:#?}",
            missing
        );
    }
}
