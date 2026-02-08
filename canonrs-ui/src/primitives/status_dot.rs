//! @canon-level: strict
//! @canon-owner: primitives-team
//! StatusDot Primitive - Visual status indicator

use leptos::prelude::*;

#[component]
pub fn StatusDotPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] variant: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-status-dot=""
            attr:data-variant={(!variant.is_empty()).then(|| variant)}
            role="status"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}
