//! CanonRS Interaction Engine
//! Tree-shaking via feature flags: ix_gesture, ix_overlay, ix_selection, ix_nav, ix_data, ix_content


// ── ix_overlay ────────────────────────────────────────────────────────────────
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::modal;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::drawer;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::sheet;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::alert_dialog;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::dialog;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::confirm_dialog;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::popover;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::hover_card;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::context_menu;
#[cfg(feature = "ix_overlay")]
pub use canonrs_interactions_overlay::dropdown_menu;

// ── ix_selection ──────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub use canonrs_interactions_selection::select;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub use canonrs_interactions_selection::combobox;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub use canonrs_interactions_selection::color_picker;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub use canonrs_interactions_selection::radio;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub use canonrs_interactions_selection::toggle_group;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub use canonrs_interactions_selection::tree;

// ── ix_nav ────────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub use canonrs_interactions_nav::sidebar;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub use canonrs_interactions_nav::menubar;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub use canonrs_interactions_nav::toolbar;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub use canonrs_interactions_nav::breadcrumb;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub use canonrs_interactions_nav::link_group;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub use canonrs_interactions_nav::pagination;

// ── ix_data ───────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub use canonrs_interactions_data::data_table;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub use canonrs_interactions_data::table;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub use canonrs_interactions_data::virtual_list;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub use canonrs_interactions_data::list_item;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub use canonrs_interactions_data::chart;

// ── ix_content ────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_content"))]
pub use canonrs_interactions_content::markdown;
#[cfg(all(target_arch = "wasm32", feature = "ix_content"))]
pub use canonrs_interactions_content::copy_button;
