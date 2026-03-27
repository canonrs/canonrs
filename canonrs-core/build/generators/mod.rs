//! generators — re-exports de todos os sub-geradores

pub use super::types;
pub use super::utils;
pub use super::parsers;

pub mod gen_schema;
pub mod gen_audit;
pub mod gen_meta;
pub mod gen_catalog;
pub mod gen_definitions;
pub mod gen_api;
pub mod gen_component_definitions;
pub use gen_component_definitions::generate_component_definitions;
pub use gen_api::{generate_api_files, generate_api_files_blocks, generate_api_files_layouts};
pub mod gen_llm;

pub use gen_schema::generate_schema_json;
pub use gen_audit::generate_audit;
pub use gen_meta::{generate_component_meta, generate_block_meta, generate_mod_update};
pub use gen_catalog::generate_catalog;
pub use gen_definitions::{generate_block_definitions, generate_layout_definitions};

pub use gen_llm::generate_llm_context;
