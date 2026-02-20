pub mod header;
pub use header::Header;
pub mod footer;
pub use footer::Footer;

// CanonRS blocks

// Core blocks
pub mod card;
pub use card::{Card, CardVariant};

pub mod alert;
pub use alert::{AlertBlock, AlertVariant};


// All other blocks
pub mod breadcrumb;
pub use breadcrumb::*;

pub mod button_group;
pub use button_group::*;

pub mod callout;
pub use callout::*;

pub mod code_block;
pub use code_block::*;

pub mod command_panel;
pub use command_panel::*;

pub mod data_table;
pub use data_table::*;

pub mod dialog;
pub use dialog::*;

pub mod drawer;
pub use drawer::*;

pub mod empty_state;
pub use empty_state::*;

pub mod field;
pub use field::*;

pub mod form;
pub use form::*;

pub mod form_actions;
pub use form_actions::*;

pub mod list;
pub use list::*;

pub mod markdown_surface;

pub mod popover;
pub use popover::*;

pub mod skeleton;
pub use skeleton::*;

pub mod stat_card;
pub use stat_card::*;

pub mod table;
pub use table::*;

pub mod toolbar;
pub use toolbar::*;

pub mod page_header;
pub use page_header::*;
