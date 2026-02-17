#![cfg(feature = "hydrate")]
use super::*;
pub fn register_all_behaviors() {
    accordion_behavior::register();
    breadcrumb_behavior::register();
    collapsible_behavior::register();
    combobox_behavior::register();
    command_behavior::register();
    context_menu_behavior::register();
    dialog_behavior::register();
    drag_drop_behavior::register();
    resize_handle_behavior::register();
    drawer_behavior::register();
    dropdown_menu_behavior::register();
    hover_card_behavior::register();
    menubar_behavior::register();
    modal_behavior::register();
    popover_behavior::register();
    select_behavior::register();
    slider_behavior::register();
    sheet_behavior::register();
    switch_behavior::register();
    theme_toggle_behavior::register();
    toggle_behavior::register();
    tooltip_behavior::register();
    tree_behavior::register();
    data_table_behavior::register();
    copy_button_behavior::register();
    markdown_behavior::register();
toc_behavior::register();
    chart_behavior::register();
    chart_sync_behavior::register();
    markdown_toolbar_behavior::register();
    doc_progress_behavior::register();
}
pub fn init_canonrs_behaviors() {
    register_all_behaviors();
    behavior_registry::init_behavior_registry();
}
