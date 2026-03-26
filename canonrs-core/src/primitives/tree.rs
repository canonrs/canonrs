//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tree Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, VisibilityState, DisabledState, ActivityState};
use crate::state_engine::{selection_attrs, visibility_attrs, disabled_attrs, activity_attrs};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TreeSelectionMode {
    #[default]
    Single,
    Multiple,
}
impl TreeSelectionMode {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Single => "single", Self::Multiple => "multiple" }
    }
    pub fn aria_multiselectable(&self) -> Option<&'static str> {
        match self { Self::Multiple => Some("true"), Self::Single => None }
    }
}

#[component]
pub fn TreePrimitive(
    children: Children,
    #[prop(default = TreeSelectionMode::Single)] selection: TreeSelectionMode,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, optional)] active_item_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tree=""
            data-rs-component="Tree"
            data-rs-behavior="navigation"
            data-rs-selection=selection.as_str()
            role="tree"
            aria-label=aria_label
            aria-multiselectable=selection.aria_multiselectable()
            aria-activedescendant=active_item_id
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TreeItemPrimitive(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = ActivityState::Inactive)] focused: ActivityState,
    #[prop(default = VisibilityState::Closed)] expanded: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0)] depth: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = selection_attrs(selected);
    let f = activity_attrs(focused);
    let e = visibility_attrs(expanded);
    let d = disabled_attrs(disabled);
    // roving tabindex: focused item = 0, rest = -1
    let tabindex = if f.data_rs_state == "active" { "0" } else { "-1" };
    view! {
        <div
            data-rs-tree-item=""
            data-rs-state=s.data_rs_state
            data-rs-focused=f.data_rs_state
            data-rs-expanded={if has_children { Some(e.data_rs_state) } else { None }}
            data-rs-disabled=d.data_rs_disabled
            data-rs-depth=depth.to_string()
            role="treeitem"
            tabindex=tabindex
            aria-selected=s.aria_selected
            aria-expanded={if has_children { Some(e.aria_expanded) } else { None }}
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TreeGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-tree-group="" role="group" class=class>
            {children()}
        </div>
    }
}
