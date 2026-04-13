//! Shared types for build.rs modules

#[derive(Debug, Clone)]
pub(crate) struct PrimitiveInfo {
    pub id:             String,
    pub component_name: String,
    pub behavior:       String,
    pub variants:       Vec<VariantInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct VariantInfo {
    pub enum_name: String,
    pub values:    Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct SemanticEntry {
    pub id:               String,
    pub label:            String,
    pub description:      String,
    pub family:           String,
    pub intent:           String,
    pub capabilities:     Vec<String>,
    #[allow(dead_code)] pub catalog_tags:     Vec<String>,
    pub catalog_category: String,
    #[allow(dead_code)] #[serde(default)] pub required_parts:  Vec<String>,
    #[allow(dead_code)] #[serde(default)] pub optional_parts:  Vec<String>,
    #[allow(dead_code)] #[serde(default)] pub composable:      bool,
    #[allow(dead_code)] #[serde(default)] pub requires_config: bool,
}


#[derive(Debug, Clone)]
pub(crate) struct BlockInfo {
    pub id:          String,
    pub kind:        String,
    pub category:    String,
    pub variant:     String,
    pub container:   bool,
    pub regions:     Vec<String>,
    #[allow(dead_code)] pub label:       Option<String>,
    #[allow(dead_code)] pub description: Option<String>,
    #[allow(dead_code)] pub tags:        Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PropInfo {
    pub key:     String,
    pub label:   String,
    pub field:   String,
    pub default: Option<String>,
    pub scope:   String,
    pub css:     Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PresetInfo {
    pub label: String,
    pub props: Vec<(String, String)>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ShowcaseEntry {
    pub id:          String,
    pub label:       String,
    pub category:    String,
    pub description: String,
    pub keywords:    String,
    pub pain:        String,
    pub promise:     String,
    pub why:         String,
    pub before:      String,
    pub after:       String,
    pub rules:       Vec<String>,
    pub use_cases:   Vec<String>,
    pub related:     Vec<String>,
    pub badges:      Vec<String>,
    pub pillar:      String,
    pub primitive_src: String,
    pub ui_src:        String,
    pub island_src:       String,
    pub block:             Vec<String>,
    pub blocks_primitives: Vec<String>,
}
