use leptos::prelude::*;
use canonrs_core::primitives::LabelPrimitive;

#[island]
pub fn LabelIsland(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <LabelPrimitive
            html_for=for_id.unwrap_or_default()
            required=required.unwrap_or(false)
            class=class.unwrap_or_default()
        >
            {text.unwrap_or_default()}
        </LabelPrimitive>
    }
}
