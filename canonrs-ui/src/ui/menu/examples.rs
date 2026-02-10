use leptos::prelude::*;
use super::menu_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Menu id="menu-basic".to_string() aria_label="Main menu".to_string()>
            <MenuGroup aria_label="Account".to_string()>
                <MenuLabel>"Account"</MenuLabel>
                <MenuItem>"Profile"</MenuItem>
                <MenuItem>"Settings"</MenuItem>
            </MenuGroup>
            <MenuSeparator />
            <MenuItem>"Logout"</MenuItem>
        </Menu>
    }
}
