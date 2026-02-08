#![cfg(feature = "hydrate")]
use super::*;

pub fn register_all_behaviors() {
    accordion_behavior::register();
    dialog_behavior::register();
    checkbox_behavior::register();
    collapsible_behavior::register();
    switch_behavior::register();
    theme_toggle_behavior::register();
    toggle_behavior::register();
}

pub fn init_canonrs_behaviors() {
    register_all_behaviors();
    behavior_registry::init_behavior_registry();
}
