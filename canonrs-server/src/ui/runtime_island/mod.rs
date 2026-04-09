//! RuntimeIsland — bootstrap único para todos os componentes interativos.
//! Incluído uma vez no layout global. Substitui todos os *Init islands.

use leptos::prelude::*;

#[island]
pub fn RuntimeIsland() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        canonrs_client::runtime::init();
    }
    view! { <></> }
}
