//! @canon-level: strict
//! @canon-owner: primitives-team
//! HiddenInput Primitive - substitui <input type="hidden">

use leptos::prelude::*;

#[component]
pub fn HiddenInputPrimitive(
    #[prop(into)] name: String,
    #[prop(into, default = String::new())] value: String,
) -> impl IntoView {
    view! {
        <input
            data-rs-hidden-input=""
            type="hidden"
            name=name
            value=value
        />
    }
}
