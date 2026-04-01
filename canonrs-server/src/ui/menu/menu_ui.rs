
use leptos::prelude::*;
use canonrs_core::primitives::{
    MenuPrimitive, MenuItemPrimitive, MenuGroupPrimitive,
    MenuLabelPrimitive, MenuSeparatorPrimitive,
};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn Menu(
    children: Children,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuPrimitive aria_label=aria_label.unwrap_or_default() class=class>
            {children()}
        </MenuPrimitive>
    }
}

#[component]
pub fn MenuItem(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuItemPrimitive disabled=disabled selected=selected class=class>
            {children()}
        </MenuItemPrimitive>
    }
}

#[component]
pub fn MenuGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuGroupPrimitive class=class>
            {children()}
        </MenuGroupPrimitive>
    }
}

#[component]
pub fn MenuLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuLabelPrimitive class=class>
            {children()}
        </MenuLabelPrimitive>
    }
}

#[component]
pub fn MenuSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuSeparatorPrimitive class=class />
    }
}

#[component]
pub fn MenuPreview() -> impl IntoView {
    view! {
        <Menu>
            <MenuItem>"Item 1"</MenuItem>
            <MenuItem>"Item 2"</MenuItem>
            <MenuSeparator />
            <MenuItem disabled=DisabledState::Disabled>"Disabled"</MenuItem>
        </Menu>
    }
}
