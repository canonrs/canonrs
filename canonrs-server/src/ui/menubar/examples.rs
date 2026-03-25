use leptos::prelude::*;
use super::menubar_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Menubar>
            <MenubarMenu>
                <MenubarTrigger>"File"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"New"</MenubarItem>
                    <MenubarItem>"Open"</MenubarItem>
                    <MenubarSeparator />
                    <MenubarItem>"Save"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
            <MenubarMenu>
                <MenubarTrigger>"Edit"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"Undo"</MenubarItem>
                    <MenubarItem>"Redo"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    }
}
