//! generators — re-exports de todos os sub-geradores

pub(crate) use super::types;
pub(crate) use super::utils;
pub(crate) use super::parsers;

pub(crate) mod gen_schema;
pub(crate) mod gen_audit;
pub(crate) mod gen_meta;
pub(crate) mod gen_catalog;
pub(crate) mod gen_definitions;
pub(crate) mod gen_api;
pub(crate) mod gen_component_definitions;
pub(crate) use gen_component_definitions::generate_component_definitions;
pub(crate) use gen_api::{generate_api_files, generate_api_files_blocks, generate_api_files_layouts, generate_api_files_layout_primitives, generate_ui_exports_doc};
pub(crate) mod gen_llm;

pub(crate) use gen_schema::generate_schema_json;
pub(crate) use gen_audit::generate_audit;
pub(crate) use gen_meta::{generate_component_meta, generate_block_meta};
pub(crate) use gen_catalog::generate_catalog;
pub(crate) use gen_definitions::{generate_block_definitions, generate_layout_definitions};

pub(crate) use gen_llm::generate_llm_context;
pub(crate) mod gen_rules;
pub(crate) use gen_rules::{parse_rules, generate_rules_json, generate_rules_seo, generate_rules_llm};

pub(crate) mod gen_showcase;
pub(crate) use gen_showcase::generate_showcase;
