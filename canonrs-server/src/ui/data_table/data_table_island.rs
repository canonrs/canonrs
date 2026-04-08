//! DataTable Island — Canon Rule #342
//! Bootstrap only. Toda a lógica vive em canonrs-client/src/interactions/data_table.rs

use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataTableIslandColumn {
    pub key:   String,
    pub label: String,
}

#[island]
pub fn DataTableInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }

    view! { <></> }
}
