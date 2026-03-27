//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavItem Primitive - Item atomico de navegacao

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};
use crate::infra::state_engine::{activity_attrs, disabled_attrs};

#[component]
pub fn NavItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    let a = activity_attrs(active);
    let d = disabled_attrs(disabled);
    let is_active = active == ActivityState::Active;
    view! {
        <a
            data-rs-nav-item=""
            data-rs-component="NavItem"
            data-rs-behavior="navigation"
            data-rs-state=a.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-current=if is_active { Some("page") } else { None }
            aria-label=aria_label
            aria-disabled=d.aria_disabled
            href=if d.disabled { "#".to_string() } else { href }
            tabindex=if d.disabled { "-1" } else { "0" }
            class=class
        >
            {children()}
        </a>
    }
}
