use leptos::prelude::*;
use crate::utils::id_gen::gen_toggle_id;

#[component]
pub fn Toggle(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] pressed: bool,
    #[prop(optional)] aria_label: Option<String>,
    children: Children,
) -> impl IntoView {
    let resolved_id = if id.is_empty() { gen_toggle_id() } else { id };
    let state = if pressed { "on" } else { "off" };

    view! {
        <button
            id={resolved_id}
            class={class}
            data-toggle
            data-state={state}
            aria-label={aria_label}
        >
            {children()}
        </button>
    }
}
