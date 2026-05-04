//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tree Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{SelectionState, DisabledState, ActivityState};

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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_tr = crate::infra::uid::generate("tr");
    view! {
        <div
            data-rs-tree=""
            data-rs-uid=uid_tr
            data-rs-interaction="selection"
            data-rs-selection=selection.as_str()
            role="tree"
            aria-label=aria_label
            aria-multiselectable=selection.aria_multiselectable()
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
    #[prop(default = false)] expanded: bool,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0u8)] depth: u8,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let tabindex = if focused.as_str() == "active" { "0" } else { "-1" };
    let depth_str = depth.to_string();
    view! {
        <div
            data-rs-tree-item=""
            data-rs-selection=if selected == SelectionState::Selected { Some("selected") } else { None }
            data-rs-focused=focused.as_str()
            data-rs-expanded={if has_children { Some(if expanded { "true" } else { "false" }) } else { None }}
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-depth=depth_str
            role="treeitem"
            tabindex=tabindex
            aria-selected=if selected == SelectionState::Selected { Some("true") } else { None }
            aria-expanded={if has_children { Some(if expanded { "true" } else { "false" }) } else { None }}
            aria-disabled=disabled.aria_disabled()
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
