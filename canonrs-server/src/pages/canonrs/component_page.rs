use leptos::prelude::*;
// use crate::blocks::canonrs::pillars_strip::{CANON_PILLARS, PillarsStrip};

// #[cfg(target_arch = "wasm32")]
// use crate::blocks::canonrs::pillars_strip::PillarsController;

#[component]
pub fn ComponentPage(children: Children) -> impl IntoView {
    view! {
        <div class="component-page">
            {children()}
        </div>
    }
}
