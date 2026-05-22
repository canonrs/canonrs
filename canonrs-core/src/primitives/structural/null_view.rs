//! @canon-level: strict
//! @canon-owner: primitives-team
//! NullView Primitive - substitui <span hidden=true/> e <span/>

use leptos::prelude::*;

/// Renderiza nada. Substitui view! { <span hidden=true/> } e view! { <span/> }
/// como placeholder de branch vazia em match/if.
#[component]
pub fn NullViewPrimitive() -> impl IntoView {
    view! {
        <span data-rs-null="" hidden=true aria-hidden="true" />
    }
}
