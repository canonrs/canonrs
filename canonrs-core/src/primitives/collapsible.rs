//! @canon-level: strict
//! @canon-owner: primitives-team
//! Collapsible Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, DisabledState};
use crate::infra::state_engine::{visibility_attrs, trigger_attrs, disabled_attrs};

fn collapsible_uid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    format!("co-{:016x}-{:08x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(ctr),
        ctr)
}

#[component]
pub fn CollapsiblePrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let s = visibility_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-collapsible=""
            data-rs-uid=collapsible_uid()
            data-rs-interaction="init"
            data-rs-component="Collapsible"
            data-rs-behavior="disclosure"
            data-rs-state=s.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-disabled=d.aria_disabled
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsibleTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let t = trigger_attrs(state);
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-collapsible-trigger=""
            data-rs-state=t.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-expanded=t.aria_expanded
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CollapsibleContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-collapsible-content=""
            data-rs-state=s.data_rs_state
            role="region"
            aria-hidden=s.aria_hidden
            class=class
        >
            {children()}
        </div>
    }
}
