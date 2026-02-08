use leptos::prelude::*;
use super::dropdown_menu_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <DropdownMenu id="dropdown-ex".to_string()>
                <DropdownMenuTrigger 
                    target_dropdown_id="dropdown-ex".to_string()
                    id="dropdown-trigger-ex".to_string()
                >
                    "Options ▼"
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
