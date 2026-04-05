use leptos::prelude::*;
use super::menubar_island::{MenubarIsland, MenubarIslandMenu, MenubarIslandItem};

fn file_menu() -> MenubarIslandMenu {
    MenubarIslandMenu {
        trigger: "File".into(),
        items: vec![
            MenubarIslandItem { label: "New".into(),  value: "new".into(),  disabled: false },
            MenubarIslandItem { label: "Open".into(), value: "open".into(), disabled: false },
            MenubarIslandItem { label: "Save".into(), value: "save".into(), disabled: false },
            MenubarIslandItem { label: "Exit".into(), value: "exit".into(), disabled: false },
        ],
    }
}

fn edit_menu() -> MenubarIslandMenu {
    MenubarIslandMenu {
        trigger: "Edit".into(),
        items: vec![
            MenubarIslandItem { label: "Cut".into(),   value: "cut".into(),   disabled: false },
            MenubarIslandItem { label: "Copy".into(),  value: "copy".into(),  disabled: false },
            MenubarIslandItem { label: "Paste".into(), value: "paste".into(), disabled: false },
            MenubarIslandItem { label: "Find".into(),  value: "find".into(),  disabled: false },
        ],
    }
}

fn view_menu() -> MenubarIslandMenu {
    MenubarIslandMenu {
        trigger: "View".into(),
        items: vec![
            MenubarIslandItem { label: "Zoom in".into(),     value: "zoom-in".into(),     disabled: false },
            MenubarIslandItem { label: "Zoom out".into(),    value: "zoom-out".into(),     disabled: false },
            MenubarIslandItem { label: "Full screen".into(), value: "fullscreen".into(),   disabled: false },
        ],
    }
}

fn help_menu() -> MenubarIslandMenu {
    MenubarIslandMenu {
        trigger: "Help".into(),
        items: vec![
            MenubarIslandItem { label: "Documentation".into(), value: "docs".into(),  disabled: false },
            MenubarIslandItem { label: "About".into(),         value: "about".into(), disabled: false },
        ],
    }
}

#[component]
pub fn MenubarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <MenubarIsland menus=vec![file_menu(), edit_menu(), view_menu(), help_menu()] />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Menubar semantics and structure governed by signal — SSR-safe, hydration-safe."
            </p>
        </div>
    }
}
