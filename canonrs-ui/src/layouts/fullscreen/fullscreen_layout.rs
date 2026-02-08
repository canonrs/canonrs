//! # FullscreenLayout
//! 
//! **Purpose:** Focused experiences, builders, editors, tools
//! **Coverage:** Theme Builder, Page Builder, Editors, Canvas, Studio
//! 
//! **Structure:**
//! ```
//! Optional Header (minimal)
//! Fullscreen Content (100%)
//! ```
//! 
//! **Token Family:** Layout (layout.fullscreen.*)

use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    /// Optional minimal header
    #[prop(optional)]
    header: Option<Children>,
    /// Whether to show header
    #[prop(default = false)]
    show_header: bool,
    /// Main fullscreen content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="layout-fullscreen"
            attr:data-layout="fullscreen"
            attr:data-has-header=show_header
        >
            {show_header.then(|| header.map(|h| view! {
                <header
                    class="layout-fullscreen-header"
                    attr:data-layout-region="header"
                >
                    {h()}
                </header>
            }))}

            <main
                class="layout-fullscreen-content"
                attr:data-layout-region="content"
            >
                {children()}
            </main>
        </div>
    }
}
