//! Constraint Engine — valida composição e decide aceitação
//! Opera sobre dados estáticos gerados (BlockDefinition, ComponentMeta, CatalogEntry)

use crate::block_types::{BlockDefinition, BlockCategory, AcceptRule};
use crate::catalog_types::{CatalogEntry, CatalogKind};
use crate::meta_types::ComponentMeta;

// ── Resultado de validação ────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(Vec<ConstraintViolation>),
}

#[derive(Debug, PartialEq)]
pub enum ConstraintViolation {
    /// Region não aceita este bloco
    RegionRejectsBlock { region: String, block_id: String },
    /// Bloco não é container mas recebeu filhos
    NotAContainer { block_id: String },
    /// Region excedeu max_children
    RegionFull { region: String, max: usize },
    /// Required parts ausentes
    MissingParts { component: String, missing: Vec<&'static str> },
    /// Bloco não existe no catálogo
    UnknownBlock { block_id: String },
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
    pub fn violations(&self) -> &[ConstraintViolation] {
        match self {
            ValidationResult::Valid => &[],
            ValidationResult::Invalid(v) => v,
        }
    }
}

// ── Engine principal ──────────────────────────────────────────────────────────

pub struct ConstraintEngine;

impl ConstraintEngine {
    /// Verifica se um bloco pode ser inserido numa region de um parent
    pub fn can_insert(
        parent_id: &str,
        region_id: &str,
        child_id:  &str,
        current_children: usize,
    ) -> ValidationResult {
        let parent = match BlockDefinition::find(parent_id) {
            Some(d) => d,
            None    => return ValidationResult::Invalid(vec![
                ConstraintViolation::UnknownBlock { block_id: parent_id.to_string() }
            ]),
        };

        if !parent.is_container {
            return ValidationResult::Invalid(vec![
                ConstraintViolation::NotAContainer { block_id: parent_id.to_string() }
            ]);
        }

        let region = match parent.find_region(region_id) {
            Some(r) => r,
            None    => return ValidationResult::Invalid(vec![
                ConstraintViolation::RegionRejectsBlock {
                    region: region_id.to_string(),
                    block_id: child_id.to_string(),
                }
            ]),
        };

        // Verifica max_children
        if let Some(max) = region.max_children {
            if current_children >= max {
                return ValidationResult::Invalid(vec![
                    ConstraintViolation::RegionFull { region: region_id.to_string(), max }
                ]);
            }
        }

        // Verifica AcceptRule
        let child_category = Self::resolve_category(child_id);
        let accepted = region.accepts.iter().any(|rule| match rule {
            AcceptRule::Any           => true,
            AcceptRule::Category(c)   => Some(*c) == child_category,
            AcceptRule::Block(id)     => *id == child_id,
        });

        if accepted {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(vec![
                ConstraintViolation::RegionRejectsBlock {
                    region: region_id.to_string(),
                    block_id: child_id.to_string(),
                }
            ])
        }
    }

    /// Verifica se um componente tem todos os required_parts
    pub fn validate_parts(component_id: &str, provided_parts: &[&str]) -> ValidationResult {
        let meta = match Self::resolve_meta(component_id) {
            Some(m) => m,
            None    => return ValidationResult::Invalid(vec![
                ConstraintViolation::UnknownBlock { block_id: component_id.to_string() }
            ]),
        };
        let missing: Vec<&'static str> = meta.required_parts
            .iter()
            .filter(|&&req| !provided_parts.contains(&req))
            .copied()
            .collect();
        if missing.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(vec![
                ConstraintViolation::MissingParts {
                    component: component_id.to_string(),
                    missing,
                }
            ])
        }
    }

    /// Resolve a categoria de um bloco pelo id
    pub fn resolve_category(block_id: &str) -> Option<BlockCategory> {
        BlockDefinition::find(block_id).map(|d| d.category)
    }

    /// Resolve o meta de um componente pelo id
    pub fn resolve_meta(component_id: &str) -> Option<&'static ComponentMeta> {
        crate::generated::component_definitions::COMPONENT_DEFINITIONS_GENERATED
            .iter()
            .find(|d| d.id == component_id)
            .map(|d| d.meta)
    }

    /// Lista todos os blocos que podem ser inseridos numa region
    pub fn valid_children_for(parent_id: &str, region_id: &str) -> Vec<&'static str> {
        let parent = match BlockDefinition::find(parent_id) {
            Some(d) => d,
            None    => return vec![],
        };
        let region = match parent.find_region(region_id) {
            Some(r) => r,
            None    => return vec![],
        };

        crate::generated::block_definitions::BLOCK_DEFINITIONS_GENERATED
            .iter()
            .filter(|b| region.accepts.iter().any(|rule| match rule {
                AcceptRule::Any           => true,
                AcceptRule::Category(c)   => *c == b.category,
                AcceptRule::Block(id)     => *id == b.id,
            }))
            .map(|b| b.id)
            .collect()
    }

    /// Verifica se child pode entrar numa region específica do parent — via region_rules do catalog
    pub fn catalog_can_nest_in_region(parent: &CatalogEntry, region_id: &str, child: &CatalogEntry) -> bool {
        use crate::catalog_types::CatalogAcceptRule;
        // Tenta region_rules primeiro (mais específico)
        if let Some(region_rule) = parent.region_rules.iter().find(|r| r.region == region_id) {
            return region_rule.accepts.iter().any(|rule| match rule {
                CatalogAcceptRule::Any             => true,
                CatalogAcceptRule::AnyComponent    => matches!(child.kind, CatalogKind::Component),
                CatalogAcceptRule::AnyBlock        => matches!(child.kind, CatalogKind::Block),
                CatalogAcceptRule::AnyLayout       => matches!(child.kind, CatalogKind::Layout),
                CatalogAcceptRule::ComponentCategory(cat) => {
                    matches!(child.kind, CatalogKind::Component) && child.category == *cat
                }
                CatalogAcceptRule::BlockCategory(cat) => {
                    matches!(child.kind, CatalogKind::Block) && child.category == *cat
                }
                CatalogAcceptRule::Only(id)        => child.id == *id,
                CatalogAcceptRule::None            => false,
            });
        }
        // Fallback para accepts global
        Self::catalog_can_nest(parent, child)
    }

    /// Verifica se um CatalogEntry pode ser filho de outro — data-driven via catalog
    pub fn catalog_can_nest(parent: &CatalogEntry, child: &CatalogEntry) -> bool {
        use crate::catalog_types::CatalogAcceptRule;
        if parent.accepts.is_empty() { return false; }
        parent.accepts.iter().any(|rule| match rule {
            CatalogAcceptRule::Any             => true,
            CatalogAcceptRule::AnyComponent    => matches!(child.kind, CatalogKind::Component),
            CatalogAcceptRule::AnyBlock        => matches!(child.kind, CatalogKind::Block),
            CatalogAcceptRule::AnyLayout       => matches!(child.kind, CatalogKind::Layout),
            CatalogAcceptRule::ComponentCategory(cat) => {
                matches!(child.kind, CatalogKind::Component) && child.category == *cat
            }
            CatalogAcceptRule::BlockCategory(cat) => {
                matches!(child.kind, CatalogKind::Block) && child.category == *cat
            }
            CatalogAcceptRule::Only(id)        => child.id == *id,
            CatalogAcceptRule::None            => false,
        })
    }

    /// Verifica se um id de catalog pode ser filho de outro id — lookup direto
    pub fn catalog_id_can_nest(parent_id: &str, child_id: &str) -> bool {
        let parent = CatalogEntry::find(parent_id);
        let child  = CatalogEntry::find(child_id);
        match (parent, child) {
            (Some(p), Some(c)) => Self::catalog_can_nest(p, c),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_accepts_content() {
        let result = ConstraintEngine::can_insert("card", "content", "button", 0);
        assert!(result.is_valid(), "{:?}", result);
    }

    #[test]
    fn test_unknown_parent() {
        let result = ConstraintEngine::can_insert("nonexistent", "content", "button", 0);
        assert!(!result.is_valid());
    }

    #[test]
    fn test_unknown_region() {
        let result = ConstraintEngine::can_insert("card", "nonexistent_region", "button", 0);
        assert!(!result.is_valid());
    }

    #[test]
    fn test_validate_parts_accordion_valid() {
        let result = ConstraintEngine::validate_parts(
            "accordion",
            &["AccordionItem", "AccordionTrigger", "AccordionContent"],
        );
        assert!(result.is_valid(), "{:?}", result);
    }

    #[test]
    fn test_validate_parts_accordion_missing() {
        let result = ConstraintEngine::validate_parts(
            "accordion",
            &["AccordionItem"],
        );
        assert!(!result.is_valid());
        match result {
            ValidationResult::Invalid(violations) => {
                assert!(violations.iter().any(|v| matches!(v, ConstraintViolation::MissingParts { .. })));
            }
            _ => panic!("expected Invalid"),
        }
    }

    #[test]
    fn test_valid_children_for_card() {
        let children = ConstraintEngine::valid_children_for("card", "content");
        assert!(!children.is_empty(), "card/content should accept children");
    }

    #[test]
    fn test_header_region_rejects_data() {
        // header aceita Category(Page) e Category(Layout) — não Data
        let result = ConstraintEngine::can_insert("dashboard", "header", "data-table", 0);
        // data-table é BlockCategory::Data — não deve ser aceito no header
        assert!(!result.is_valid(), "header should not accept data-table");
    }

    #[test]
    fn test_catalog_cannot_nest_layout_in_layout() {
        use crate::catalog_types::{CatalogEntry, CatalogCategory, CatalogKind};
        let parent = CatalogEntry {
            id: "dashboard", label: "Dashboard", description: "",
            kind: CatalogKind::Layout, category: CatalogCategory::Layout,
            tags: &[], parts: &[], regions: &["header", "sidebar", "content"], accepts: &[], region_rules: &[],
        };
        let child = CatalogEntry {
            id: "marketing", label: "Marketing", description: "",
            kind: CatalogKind::Layout, category: CatalogCategory::Layout,
            tags: &[], parts: &[], regions: &[], accepts: &[], region_rules: &[],
        };
        assert!(!ConstraintEngine::catalog_can_nest(&parent, &child));
    }
}
