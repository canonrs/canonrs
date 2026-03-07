use leptos::prelude::*;

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label data-toggle="" class={class}>
            <input
                type="checkbox"
                id={if id.is_empty() { None } else { Some(id) }}
            />
            <span data-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
