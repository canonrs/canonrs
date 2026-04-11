//! @canon-owner: primitives-team
//! Accordion Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState};
use crate::infra::state_engine::visibility_attrs;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Default, Debug)]
pub enum AccordionSelection {
    #[default]
    Single,
    Multiple,
}
impl AccordionSelection {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Single => "single", Self::Multiple => "multiple" }
    }
}

fn accordion_uid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    format!("ac-{:016x}-{:08x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(ctr),
        ctr)
}

#[component]
pub fn AccordionPrimitive(
    children: Children,
    #[prop(default = AccordionSelection::Single)] selection: AccordionSelection,
    #[prop(default = true)] collapsible: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion=""
            data-rs-uid=accordion_uid()
            data-rs-interaction="nav"
            data-rs-component="Accordion"
            data-rs-selection=selection.as_str()
            data-rs-collapsible=if collapsible { "true" } else { "false" }
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItemPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let state_str = if disabled == DisabledState::Disabled {
        format!("{} disabled", s.data_rs_state)
    } else {
        s.data_rs_state.to_string()
    };
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <div
            data-rs-accordion-item=""
            data-rs-state=state_str
            aria-disabled=aria_disabled
            role="group"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTriggerPrimitive(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <h3 data-rs-accordion-heading="">
            <button
                type="button"
                data-rs-accordion-trigger=""
                aria-expanded="false"
                aria-disabled=aria_disabled
                class=class
            >
                {children()}
            </button>
        </h3>
    }
}

#[component]
pub fn AccordionContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-accordion-content=""
            class=class
        >
            {children()}
        </div>
    }
}
