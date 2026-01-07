use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenRow {
    pub id: String,
    pub scope: String,
    pub domain: String,
    pub family_code: Option<String>,
    pub family_name: Option<String>,
    pub category: String,
    pub name: String,
    pub full_name: String,
    pub value: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ComponentRow {
    pub id: String,
    pub tipo: String,
    pub name: String,
    pub familia: String,
    pub canonical_type: String,
    pub status: String,
    pub tokens_canonicos_percent: i32,
    pub tokens_familia_c_percent: i32,
    pub familias_aplicadas: Option<String>,
    pub has_readme: bool,
    pub folder_structure_correct: bool,
    pub canon_score: i32,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenFamilyDetailRow {
    pub family_code: String,
    pub family_name: String,
    pub token_name: String,
    pub token_full_name: String,
    pub description: Option<String>,
}
