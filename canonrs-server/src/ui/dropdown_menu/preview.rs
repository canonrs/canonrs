use leptos::prelude::*;
use super::dropdown_menu_ui::{
    DropdownMenu, DropdownMenuTrigger, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuCheckboxItem,
    DropdownMenuLabel, DropdownMenuSeparator,
};
use canonrs_core::meta::ToggleState;

#[component]
pub fn DropdownMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DropdownMenu>
                    <DropdownMenuTrigger>"Options ▼"</DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <DropdownMenuLabel>"Account"</DropdownMenuLabel>
                        <DropdownMenuGroup>
                            <DropdownMenuItem>"Profile"</DropdownMenuItem>
                            <DropdownMenuItem>"Settings"</DropdownMenuItem>
                            <DropdownMenuItem>"Billing"</DropdownMenuItem>
                        </DropdownMenuGroup>
                        <DropdownMenuSeparator />
                        <DropdownMenuCheckboxItem checked=ToggleState::On>"Notifications"</DropdownMenuCheckboxItem>
                        <DropdownMenuCheckboxItem>"Dark mode"</DropdownMenuCheckboxItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>"Sign out"</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menu interaction and state fully encoded via primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With groups"</span>
                <div data-rs-showcase-preview-row="">
                    <DropdownMenu>
                        <DropdownMenuTrigger>"Actions ▼"</DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuLabel>"File"</DropdownMenuLabel>
                            <DropdownMenuGroup>
                                <DropdownMenuItem>"New"</DropdownMenuItem>
                                <DropdownMenuItem>"Open"</DropdownMenuItem>
                                <DropdownMenuItem>"Save"</DropdownMenuItem>
                            </DropdownMenuGroup>
                            <DropdownMenuSeparator />
                            <DropdownMenuLabel>"Edit"</DropdownMenuLabel>
                            <DropdownMenuGroup>
                                <DropdownMenuItem>"Cut"</DropdownMenuItem>
                                <DropdownMenuItem>"Copy"</DropdownMenuItem>
                                <DropdownMenuItem>"Paste"</DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            </div>
        </div>
    }
}
