use leptos::prelude::*;
use crate::primitives::LabelPrimitive;

#[component]
pub fn Label(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] for_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <LabelPrimitive
            for_id=for_id
            class=class
            id=id
        >
            {children.map(|c| c())}
        </LabelPrimitive>
    }
}
