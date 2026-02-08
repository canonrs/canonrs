use leptos::prelude::*;
use super::dropdown_menu_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <DropdownMenu open=Signal::from(false) id="dropdown-ex".to_string()>
                <DropdownMenuTrigger open=Signal::from(false) id="dropdown-trigger-ex".to_string()>
                    <button data-button data-ui-variant="default">"Options ▼"</button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                    <DropdownMenuGroup>
                        <DropdownMenuLabel>"Account"</DropdownMenuLabel>
                        <DropdownMenuItem>"Profile"</DropdownMenuItem>
                        <DropdownMenuItem>"Settings"</DropdownMenuItem>
                    </DropdownMenuGroup>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem>"Logout"</DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    }
}
