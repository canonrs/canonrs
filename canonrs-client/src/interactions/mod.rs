//! CanonRS Interaction Engine
//! Tree-shaking via feature flags: ix_gesture, ix_overlay, ix_selection, ix_nav, ix_data, ix_content

// ── ix_gesture ────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_gesture"))]
pub mod resizable;
#[cfg(all(target_arch = "wasm32", feature = "ix_gesture"))]
pub mod slider;
#[cfg(all(target_arch = "wasm32", feature = "ix_gesture"))]
pub mod carousel;
#[cfg(all(target_arch = "wasm32", feature = "ix_gesture"))]
pub mod scroll_area;

// ── ix_overlay ────────────────────────────────────────────────────────────────
#[cfg(feature = "ix_overlay")]
pub mod modal;
#[cfg(feature = "ix_overlay")]
pub mod drawer;
#[cfg(feature = "ix_overlay")]
pub mod sheet;
#[cfg(feature = "ix_overlay")]
pub mod alert_dialog;
#[cfg(feature = "ix_overlay")]
pub mod dialog;
#[cfg(feature = "ix_overlay")]
pub mod confirm_dialog;
#[cfg(feature = "ix_overlay")]
pub mod popover;
#[cfg(feature = "ix_overlay")]
pub mod hover_card;
#[cfg(feature = "ix_overlay")]
pub mod context_menu;
#[cfg(feature = "ix_overlay")]
pub mod dropdown_menu;

// ── ix_selection ──────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub mod select;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub mod combobox;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub mod color_picker;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub mod radio;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub mod toggle_group;
#[cfg(all(target_arch = "wasm32", feature = "ix_selection"))]
pub mod tree;

// ── ix_nav ────────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub mod sidebar;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub mod menubar;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub mod toolbar;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub mod breadcrumb;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub mod link_group;
#[cfg(all(target_arch = "wasm32", feature = "ix_nav"))]
pub mod pagination;

// ── ix_data ───────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub mod data_table;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub mod table;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub mod virtual_list;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub mod list_item;
#[cfg(all(target_arch = "wasm32", feature = "ix_data"))]
pub mod chart;

// ── ix_content ────────────────────────────────────────────────────────────────
#[cfg(all(target_arch = "wasm32", feature = "ix_content"))]
pub mod markdown;
#[cfg(all(target_arch = "wasm32", feature = "ix_content"))]
pub mod copy_button;
