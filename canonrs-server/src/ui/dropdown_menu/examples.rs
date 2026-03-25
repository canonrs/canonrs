use leptos::prelude::*;
use super::dropdown_menu_ui::*;
use crate::ui::button::button_ui::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <DropdownMenu>
            <Button variant=ButtonVariant::Primary attr:data-rs-dropdown-menu-trigger="">"Options ▼"</Button>
            <DropdownMenuContent>
                <DropdownMenuItem>"Edit"</DropdownMenuItem>
                <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>"Delete"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
