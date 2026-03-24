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
            data-rs-label=""
            for=for_id
            class=class
            id=id
        >
            {children.map(|c| c())}
        </label>
    }
}
