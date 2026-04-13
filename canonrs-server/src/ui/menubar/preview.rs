use leptos::prelude::*;
use super::menubar_boundary::{
    Menubar, MenubarMenu, MenubarTrigger,
    MenubarContent, MenubarItem, MenubarSeparator,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn MenubarShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Menubar semantics and structure governed by DOM state — SSR-safe, hydration-safe."
            </p>
            <Menubar>
                <MenubarMenu>
                    <MenubarTrigger>"File"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"New"</MenubarItem>
                        <MenubarItem>"Open"</MenubarItem>
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
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>"View"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>"Zoom in"</MenubarItem>
                        <MenubarItem>"Zoom out"</MenubarItem>
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
        </Stack>
    }
}
