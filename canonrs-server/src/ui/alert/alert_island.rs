//! @canon-level: strict
//! Alert Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::alert_ui::{Alert, AlertTitle, AlertDescription, AlertCloseButton};
use canonrs_core::primitives::AlertVariant;

#[component]
pub fn AlertIsland(
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(default = false)] dismissible: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Alert variant=variant class=class>
            {title.map(|t| view! { <AlertTitle>{t}</AlertTitle> })}
            {description.map(|d| view! { <AlertDescription>{d}</AlertDescription> })}
            {dismissible.then(|| view! { <AlertCloseButton>"×"</AlertCloseButton> })}
        </Alert>
    }
}
