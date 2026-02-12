use leptos::prelude::*;

#[component]
pub fn InputGroupPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] merge_radius: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={if id.is_empty() { None } else { Some(id) }}
            class={class}
            data-input-group=""
            data-merge-radius={if merge_radius { Some("") } else { None }}
        >
            {children.map(|c| c())}
        </div>
    }
}
