use leptos::prelude::*;

#[component]
pub fn LabelPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] for_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <label
            id={id}
            class={class}
            for={for_id}
            attr:data-label=""
        >
            {children.map(|c| c())}
        </label>
    }
}
