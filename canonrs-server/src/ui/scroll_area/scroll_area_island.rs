use leptos::prelude::*;
use super::scroll_area_ui::ScrollArea;
use canonrs_core::primitives::ScrollOrientation;

#[island]
pub fn ScrollAreaInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn ScrollAreaIsland(
    children: Children,
    #[prop(optional, into)] orientation: Option<String>,
    #[prop(optional)] auto_hide: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] viewport_id: Option<String>,
) -> impl IntoView {
    let orientation_prop = match orientation.as_deref() {
        Some("horizontal") => ScrollOrientation::Horizontal,
        Some("both")       => ScrollOrientation::Both,
        _                  => ScrollOrientation::Vertical,
    };
    let cls = class.unwrap_or_default();
    view! {
        <ScrollAreaInit />
        <ScrollArea
            orientation=orientation_prop
            auto_hide=auto_hide.unwrap_or(true)
            class=cls
            viewport_id=viewport_id.unwrap_or_default()
        >
            {children()}
        </ScrollArea>
    }
}
