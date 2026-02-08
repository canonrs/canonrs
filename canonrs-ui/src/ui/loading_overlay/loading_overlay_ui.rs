use leptos::prelude::*;
use crate::primitives::LoadingOverlayPrimitive;

#[component]
pub fn LoadingOverlay(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] loading: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <LoadingOverlayPrimitive
            loading={loading}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </LoadingOverlayPrimitive>
    }
}
