use leptos::prelude::*;

#[component]
pub fn ToggleGroupPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] role: String,
    #[prop(default = false)] multiple: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            role={role}
            attr:data-toggle-group=""
            attr:data-multiple={if multiple { "true" } else { "" }}
        >
            {children.map(|c| c())}
        </div>
    }
}
