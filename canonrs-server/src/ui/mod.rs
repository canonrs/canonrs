// UI Components - Family A: Overlays only (for testing)

pub mod dialog;
pub mod sheet;
pub mod drawer;
pub mod popover;
pub mod hero;
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
pub mod toggle;
pub mod toggle_group;

// Family D - Navigation
pub mod link;
pub mod navigation_menu;
pub mod link_group;
pub mod nav_item;
pub mod page_header;
pub mod table_of_contents;
pub mod breadcrumb;
pub mod pagination;
pub mod sidebar;
pub mod toolbar;

// Family E - Feedback
pub mod alert;
pub mod alert_dialog;
pub mod confirm_dialog;
pub mod progress;
pub mod pulse;
pub mod skeleton;
pub mod spinner;
pub mod status_dot;
pub mod toast;
pub mod banner;
pub mod callout;
pub mod empty_state;
pub mod error_state;
pub mod inline_meta;
pub mod inline_notice;
pub mod loading_overlay;
pub mod empty_table;

// Family F - Data & Media
pub mod table;
pub mod data_table;
pub mod chart;
pub mod scroll_area;
pub mod avatar;
pub mod icon;
pub mod copy_button;
pub mod stat;
pub mod icon_button;
pub use icon_button::icon_button_ui::{IconButton, IconButtonVariant};
pub mod list_item;
pub mod card;
pub mod markdown;

// Family G - Composite
pub mod carousel;
pub mod resizable;

// Family H - Layout
pub mod section;
pub mod separator;
pub mod aspect_ratio;
pub mod doc_progress;
pub use doc_progress::DocProgress;

pub mod animate;
pub mod virtual_list;
pub use card::Card;
pub use separator::Separator;

pub mod logo;
pub use logo::{Logo, LogoSize, LogoVariant};
