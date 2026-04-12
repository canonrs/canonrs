//! Label Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::label_ui::Label as LabelUi;

#[component]
pub fn Label(
    children: Children,
    #[prop(into, default = String::new())] for_id: String,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! {
        <LabelUi for_id=for_id class=class>
            {children()
};
        </LabelUi>
    }
}
