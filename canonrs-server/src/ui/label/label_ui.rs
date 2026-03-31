
use leptos::prelude::*;
use canonrs_core::primitives::LabelPrimitive;

#[component]
pub fn Label(
    children: Children,
    #[prop(into, default = String::new())] for_id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LabelPrimitive html_for=for_id class=class>
            {children()}
        </LabelPrimitive>
    }
}

#[component]
pub fn LabelPreview() -> impl IntoView {
    view! {
        <Label>"Username"</Label>
    }
}
