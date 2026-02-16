// UI Components - Family A: Overlays only (for testing)

pub mod dialog;
pub mod sheet;
pub mod drawer;
pub mod popover;
pub mod hover_card;
pub mod tooltip;
pub mod dropdown_menu;
pub mod context_menu;
pub mod modal;

// Temporary: needed by blocks
pub mod button;
pub mod tabs;
pub mod code_block;
pub mod badge;
pub mod input;

pub use button::Button;
pub use badge::Badge;
pub use input::Input;
pub use tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
pub mod menu;
pub mod menubar;
pub mod tree;
pub mod checkbox;
pub mod accordion;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod radio_group;
pub mod radio;
pub mod kbd;
pub mod button_group;
pub mod color_picker;
pub mod form_error_summary;
pub mod field;
pub mod input_group;
pub mod input_otp;
pub mod textarea;
pub mod slider;
pub mod label;
pub mod switch;
pub mod select;
pub use select::*;
pub mod toggle;

// Family D - Navigation
pub mod link;
pub use link::*;
pub mod navigation_menu;
pub use navigation_menu::*;
pub mod nav_item;
pub use nav_item::*;
pub mod page_header;
pub use page_header::*;
pub mod table_of_contents;
pub use table_of_contents::*;
pub mod breadcrumb;
pub use breadcrumb::*;
pub mod pagination;
pub use pagination::*;
pub mod sidebar;
pub use sidebar::*;
pub mod toolbar;
pub use toolbar::*;

// Family E - Feedback
pub mod alert;
pub use alert::*;
pub mod alert_dialog;
pub use alert_dialog::*;
pub mod progress;
pub use progress::*;
pub mod pulse;
pub use pulse::*;
pub mod skeleton;
pub use skeleton::*;
pub mod spinner;
pub use spinner::*;
pub mod status_dot;
pub use status_dot::*;
pub mod toast;
pub use toast::*;
pub mod banner;
pub use banner::*;
pub mod callout;
pub use callout::*;
pub mod empty_state;
pub use empty_state::*;
pub mod error_state;
pub use error_state::*;
pub mod inline_notice;
pub use inline_notice::*;
pub mod loading_overlay;
pub use loading_overlay::*;
pub mod empty_table;
pub use empty_table::*;

// Family F - Data & Media
pub mod table;
pub use table::*;
pub mod data_table;
pub use data_table::*;
pub mod chart;
pub use chart::*;
pub mod scroll_area;
pub use scroll_area::*;
pub mod avatar;
pub use avatar::*;
pub mod icon;
pub use icon::*;
pub mod copy_button;
pub use copy_button::*;
pub mod stat;
pub use stat::*;
pub mod icon_button;
pub use icon_button::*;
pub mod list_item;
pub use list_item::*;
pub mod card;
pub use card::*;
pub mod markdown;
pub use markdown::*;

// Family G - Composite
pub mod carousel;
pub use carousel::*;
pub mod resizable;
pub use resizable::*;

// Family H - Layout
pub mod separator;
pub use separator::*;
pub mod aspect_ratio;
pub use aspect_ratio::*;
pub mod doc_progress;
pub use doc_progress::DocProgress;
