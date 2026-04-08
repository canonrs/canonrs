use leptos::prelude::*;
use super::menubar_island::{
    MenubarIsland, MenubarMenuIsland, MenubarTriggerIsland,
    MenubarContentIsland, MenubarItemIsland, MenubarSeparatorIsland,
};

#[component]
pub fn MenubarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <MenubarIsland>
                    <MenubarMenuIsland>
                        <MenubarTriggerIsland>"File"</MenubarTriggerIsland>
                        <MenubarContentIsland>
                            <MenubarItemIsland>"New"</MenubarItemIsland>
                            <MenubarItemIsland>"Open"</MenubarItemIsland>
                            <MenubarSeparatorIsland />
                            <MenubarItemIsland>"Exit"</MenubarItemIsland>
                        </MenubarContentIsland>
                    </MenubarMenuIsland>
                    <MenubarMenuIsland>
                        <MenubarTriggerIsland>"Edit"</MenubarTriggerIsland>
                        <MenubarContentIsland>
                            <MenubarItemIsland>"Cut"</MenubarItemIsland>
                            <MenubarItemIsland>"Copy"</MenubarItemIsland>
                            <MenubarItemIsland>"Paste"</MenubarItemIsland>
                        </MenubarContentIsland>
                    </MenubarMenuIsland>
                    <MenubarMenuIsland>
                        <MenubarTriggerIsland>"View"</MenubarTriggerIsland>
                        <MenubarContentIsland>
                            <MenubarItemIsland>"Zoom in"</MenubarItemIsland>
                            <MenubarItemIsland>"Zoom out"</MenubarItemIsland>
                            <MenubarItemIsland>"Full screen"</MenubarItemIsland>
                        </MenubarContentIsland>
                    </MenubarMenuIsland>
                    <MenubarMenuIsland>
                        <MenubarTriggerIsland>"Help"</MenubarTriggerIsland>
                        <MenubarContentIsland>
                            <MenubarItemIsland>"Documentation"</MenubarItemIsland>
                            <MenubarItemIsland>"About"</MenubarItemIsland>
                        </MenubarContentIsland>
                    </MenubarMenuIsland>
                </MenubarIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menubar semantics and structure governed by DOM state — SSR-safe, hydration-safe."
            </p>
        </div>
    }
}
