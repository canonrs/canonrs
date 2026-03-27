//! @canon-level: strict
//! @canon-owner: primitives-team
//! DomContract — valida estrutura de composição de componentes
//! Usa ComponentMeta.required_parts para validação em dev

use crate::primitives::has_meta::component_meta;

#[derive(Debug, PartialEq)]
pub enum ContractResult {
    Valid,
    MissingParts(Vec<&'static str>),
    UnknownComponent,
}

/// Valida se os parts fornecidos satisfazem o contrato do componente.
/// Usado em dev/test para garantir composição correta.
pub fn validate_structure(component: &str, provided_parts: &[&str]) -> ContractResult {
    match component_meta(component) {
        None => ContractResult::UnknownComponent,
        Some(meta) => {
            let missing: Vec<&'static str> = meta
                .required_parts
                .iter()
                .filter(|&&req| !provided_parts.contains(&req))
                .copied()
                .collect();
            if missing.is_empty() {
                ContractResult::Valid
            } else {
                ContractResult::MissingParts(missing)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_valid() {
        let result = validate_structure(
            "Dialog",
            &["DialogContent", "DialogTitle", "DialogOverlay"],
        );
        assert_eq!(result, ContractResult::Valid);
    }

    #[test]
    fn test_dialog_missing_title() {
        let result = validate_structure("Dialog", &["DialogContent"]);
        assert_eq!(result, ContractResult::MissingParts(vec!["DialogTitle"]));
    }

    #[test]
    fn test_unknown_component() {
        let result = validate_structure("Unknown", &[]);
        assert_eq!(result, ContractResult::UnknownComponent);
    }
}
