//! CatalogEntry — unified type for DecisionEngine + Builder
//! Generated statics in generated/catalog.rs
//! rs-ai-canonrs-core imports and extends with parts/accepts

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CatalogKind {
    Component,
    Block,
    Layout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CatalogCategory {
    Action,
    Display,
    Feedback,
    Form,
    Navigation,
    Overlay,
    Data,
    Layout,
    Typography,
    UI,
    Block,
}

#[derive(Clone, Debug)]
pub enum PropType {
    String,
    Bool,
    Number,
    Enum(&'static [&'static str]),
    Children,
}

#[derive(Clone, Debug)]
pub struct PropDef {
    pub name:        &'static str,
    pub kind:        PropType,
    pub required:    bool,
    pub default:     Option<&'static str>,
    pub description: &'static str,
}

#[derive(Clone, Debug)]
pub struct ComponentApi {
    pub id:          &'static str,
    pub description: &'static str,
    pub props:       &'static [PropDef],
}

#[derive(Clone, Debug)]
pub struct ComponentPart {
    pub id:    &'static str,
    pub props: Vec<PropDef>,
}

/// Regra de aceitação declarativa no catalog
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CatalogAcceptRule {
    /// Aceita qualquer componente UI
    AnyComponent,
    /// Aceita qualquer block
    AnyBlock,
    /// Aceita qualquer layout
    AnyLayout,
    /// Aceita tudo
    Any,
    /// Aceita apenas components de uma categoria específica
    ComponentCategory(CatalogCategory),
    /// Aceita apenas blocks de uma categoria específica
    BlockCategory(CatalogCategory),
    /// Aceita um item específico por id
    Only(&'static str),
    /// Não aceita nada
    None,
}

/// Regra de aceitação por região específica do catalog
#[derive(Clone, Debug)]
pub struct CatalogRegionRule {
    pub region:  &'static str,
    pub accepts: &'static [CatalogAcceptRule],
}

#[derive(Clone, Debug)]
pub struct CatalogEntry {
    pub id:          &'static str,
    pub label:       &'static str,
    pub description: &'static str,
    pub kind:        CatalogKind,
    pub category:    CatalogCategory,
    pub tags:        &'static [&'static str],
    pub parts:       &'static [&'static str],
    pub regions:     &'static [&'static str],
    pub accepts:     &'static [CatalogAcceptRule],
    pub region_rules: &'static [CatalogRegionRule],
}

impl CatalogEntry {
    pub fn matches(&self, query: &str) -> bool {
        let q = query.to_lowercase();
        self.tags.iter().any(|t| t.contains(q.as_str()) || q.contains(t))
            || self.label.to_lowercase().contains(&q)
            || self.description.to_lowercase().contains(&q)
    }

    pub fn find(id: &str) -> Option<&'static CatalogEntry> {
        crate::generated::catalog::CATALOG_GENERATED.iter().find(|e| e.id == id)
    }

    pub fn match_by_tags(tags: &[&str]) -> Vec<(&'static CatalogEntry, usize)> {
        let mut results: Vec<(&'static CatalogEntry, usize)> = crate::generated::catalog::CATALOG_GENERATED
            .iter()
            .filter_map(|e| {
                let score = tags.iter().filter(|t| e.matches(t)).count();
                if score > 0 { Some((e, score)) } else { None }
            })
            .collect();
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }

    pub fn by_category(cat: CatalogCategory) -> impl Iterator<Item = &'static CatalogEntry> {
        crate::generated::catalog::CATALOG_GENERATED.iter().filter(move |e| e.category == cat)
    }
}

/// Definição completa de um componente UI derivada dos headers @canon-*
#[derive(Clone, Debug)]
pub struct ComponentDefinition {
    pub id:          &'static str,
    pub label:       &'static str,
    pub description: &'static str,
    pub kind:        CatalogKind,
    pub meta:        &'static crate::meta_types::ComponentMeta,
}

impl ComponentDefinition {
    pub fn find(id: &str) -> Option<&'static ComponentDefinition> {
        crate::generated::component_definitions::COMPONENT_DEFINITIONS_GENERATED
            .iter()
            .find(|d| d.id == id)
    }
}
