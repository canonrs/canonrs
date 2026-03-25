//! @canon-level: ui
//! Label - sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::LabelPrimitive;

#[component]
pub fn Label(
    children: Children,
    #[prop(into, default = String::new())] for_id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <LabelPrimitive for_id=for_id class=class>
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
