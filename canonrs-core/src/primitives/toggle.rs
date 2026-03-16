use leptos::prelude::*;

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = false)] pressed: bool,
    #[prop(into, default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <label data-toggle="" data-state=if pressed {"on"} else {"off"} class={class}
            aria-label={if aria_label.is_empty() { None } else { Some(aria_label) }}>
            <input
                type="checkbox"
                prop:checked=pressed
                id={if id.is_empty() { None } else { Some(id) }}
            />
            <span data-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
