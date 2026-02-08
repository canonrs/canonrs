#[cfg(feature = "hydrate")]
mod behavior_registry;

#[cfg(feature = "hydrate")]
mod auto_init;

// === BEHAVIORS ===
#[cfg(feature = "hydrate")]
mod accordion_behavior;
#[cfg(feature = "hydrate")]
mod breadcrumb_behavior;
#[cfg(feature = "hydrate")]
mod calendar_behavior;
#[cfg(feature = "hydrate")]
mod chart_behavior;
#[cfg(feature = "hydrate")]
mod checkbox_behavior;
#[cfg(feature = "hydrate")]
mod collapsible_behavior;
#[cfg(feature = "hydrate")]
mod combobox_behavior;
#[cfg(feature = "hydrate")]
mod dialog_behavior;
#[cfg(feature = "hydrate")]
mod copy_button_behavior;
#[cfg(feature = "hydrate")]
mod datatable_behavior;
#[cfg(feature = "hydrate")]
mod icon_button_behavior;
#[cfg(feature = "hydrate")]
mod icon_toggle_behavior;
#[cfg(feature = "hydrate")]
mod list_item_behavior;
#[cfg(feature = "hydrate")]
mod navigation_menu_behavior;
#[cfg(feature = "hydrate")]
mod overlay_behavior;
#[cfg(feature = "hydrate")]
mod pagination_behavior;
#[cfg(feature = "hydrate")]
mod scroll_area_behavior;
#[cfg(feature = "hydrate")]
mod sidebar_behavior;
#[cfg(feature = "hydrate")]
mod simple_overlay_behavior;
#[cfg(feature = "hydrate")]
mod switch_behavior;
#[cfg(feature = "hydrate")]
mod tabs_behavior;
#[cfg(feature = "hydrate")]
mod theme_toggle_behavior;
#[cfg(feature = "hydrate")]
mod toggle_behavior;

#[cfg(feature = "hydrate")]
pub use behavior_registry::*;

#[cfg(feature = "hydrate")]
pub use auto_init::init_canonrs_behaviors;
