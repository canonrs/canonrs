//! @canon-level: strict
//! @canon-owner: primitives-team
//! Skeleton Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn SkeletonPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-skeleton=""
            aria-busy="true"
            aria-live="polite"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
