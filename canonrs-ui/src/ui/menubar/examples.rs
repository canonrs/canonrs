use leptos::prelude::*;
use super::menubar_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Menubar id="menubar-basic".to_string()>
            <MenubarItem>
                <MenubarTrigger controls_id="menubar-file-ex".to_string()>"File"</MenubarTrigger>
                <MenubarContent content_id="menubar-file-ex".to_string()>
                    <MenubarSubItem>"New"</MenubarSubItem>
                    <MenubarSubItem>"Open"</MenubarSubItem>
                    <MenubarSeparator />
                    <MenubarSubItem>"Save"</MenubarSubItem>
                </MenubarContent>
            </MenubarItem>
            <MenubarItem>
                <MenubarTrigger controls_id="menubar-edit-ex".to_string()>"Edit"</MenubarTrigger>
                <MenubarContent content_id="menubar-edit-ex".to_string()>
                    <MenubarSubItem>"Undo"</MenubarSubItem>
                    <MenubarSubItem>"Redo"</MenubarSubItem>
                </MenubarContent>
            </MenubarItem>
        </Menubar>
    }
}
