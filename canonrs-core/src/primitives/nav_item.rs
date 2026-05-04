//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavItem Primitive - Item atomico de navegacao

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};

#[component]
pub fn NavItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    let uid_ni = crate::infra::uid::generate("ni");
    let is_active = active == ActivityState::Active;
    view! {
        <a
            data-rs-nav-item=""
            data-rs-uid=uid_ni
            data-rs-interaction="init"
            data-rs-activity=active.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            aria-current=if is_active { Some("page") } else { None }
            aria-label=aria_label
            aria-disabled=disabled.aria_disabled()
            href=if disabled.disabled() { "#".to_string() } else { href }
            tabindex=if disabled.disabled() { "-1" } else { "0" }
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn NavGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::from("vertical"))] direction: String,
) -> impl IntoView {
    let uid_ng = crate::infra::uid::generate("ng");
    view! {
        <nav
            data-rs-nav-group=""

                        data-rs-uid=uid_ng
            data-rs-interaction="init"
            data-rs-direction=direction
            aria-label=aria_label
            class=class
        >
            {children()}
        </nav>
    }
}
