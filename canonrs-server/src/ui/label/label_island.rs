//! Label Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::label_ui::Label;

#[component]
pub fn LabelIsland(
    children: Children,
    #[prop(into, default = String::new())] for_id: String,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! {
        <Label for_id=for_id class=class>
            {children()}
        </Label>
    }
}
