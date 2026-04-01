use leptos::prelude::*;
use super::menubar_ui::{
    Menubar, MenubarMenu, MenubarTrigger, MenubarContent,
    MenubarItem, MenubarSeparator,
};

#[component]
pub fn MenubarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Menubar>
                    <MenubarMenu>
                        <MenubarTrigger>"File"</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>"New"</MenubarItem>
                            <MenubarItem>"Open"</MenubarItem>
                            <MenubarItem>"Save"</MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem>"Exit"</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                    <MenubarMenu>
                        <MenubarTrigger>"Edit"</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>"Cut"</MenubarItem>
                            <MenubarItem>"Copy"</MenubarItem>
                            <MenubarItem>"Paste"</MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem>"Find"</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                    <MenubarMenu>
                        <MenubarTrigger>"View"</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>"Zoom in"</MenubarItem>
                            <MenubarItem>"Zoom out"</MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem>"Full screen"</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                    <MenubarMenu>
                        <MenubarTrigger>"Help"</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>"Documentation"</MenubarItem>
                            <MenubarItem>"About"</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                </Menubar>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menubar semantics and structure enforced via primitives."
            </p>
        </div>
    }
}
