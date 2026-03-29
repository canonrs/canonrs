//! @canon-level: strict
//! InputGroup Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::ActivityState;

#[component]
pub fn InputGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ActivityState::Inactive)] merge_radius: ActivityState,
) -> impl IntoView {
    view! {
        <div
            data-rs-input-group=""
            data-rs-merge-radius={if merge_radius == ActivityState::Active { Some("") } else { None }}
            class=class
        >
            {children()}
        </div>
    }
}
