//! blocks::prelude — re-exports curados para consumo externo
//!
//! use canonrs_server::blocks::prelude::*;

pub use super::hero::hero_block::{Hero, HeroVariant};
pub use super::card::card_block::{CardBlock, CardVariant};
pub use super::page_header::page_header_block::PageHeader;
pub use super::sidebar_layout::sidebar_layout_block::{SidebarLayout, SidebarSide};
pub use super::section::section_block::SectionBlock;
pub use super::form_field::form_field_block::FormFieldBlock;
pub use super::stat_group::stat_group_block::StatGroupBlock;
pub use super::data_table::data_table_block::DataTableBlock;
