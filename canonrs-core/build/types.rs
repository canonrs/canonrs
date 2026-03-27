//! Shared types for build.rs modules

#[derive(Debug, Clone)]
pub struct PrimitiveInfo {
    pub id:             String,
    pub component_name: String,
    pub behavior:       String,
    pub variants:       Vec<VariantInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct VariantInfo {
    pub enum_name: String,
    pub values:    Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SemanticEntry {
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
pub struct BlockInfo {
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
pub struct PropInfo {
    pub key:     String,
    pub label:   String,
    pub field:   String,
    pub default: Option<String>,
    pub scope:   String,
    pub css:     Option<String>,
}

#[derive(Debug, Clone)]
pub struct PresetInfo {
    pub label: String,
    pub props: Vec<(String, String)>,
}
