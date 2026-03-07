use leptos::prelude::*;

#[component]
pub fn ButtonGroupPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] role: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            role={role}
            attr:data-button-group=""
        >
            {children.map(|c| c())}
        </div>
    }
}
