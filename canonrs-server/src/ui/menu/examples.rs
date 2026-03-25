use leptos::prelude::*;
use super::menu_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Menu aria_label="Main menu">
            <MenuGroup>
                <MenuLabel>"Account"</MenuLabel>
                <MenuItem>"Profile"</MenuItem>
                <MenuItem>"Settings"</MenuItem>
            </MenuGroup>
            <MenuSeparator />
            <MenuItem>"Logout"</MenuItem>
        </Menu>
    }
}
