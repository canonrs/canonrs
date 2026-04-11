//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toggle Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ToggleState, DisabledState};
use crate::infra::state_engine::{disabled_attrs, toggle_attrs};

fn toggle_uid() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static CTR: AtomicU32 = AtomicU32::new(0);
    format!("to-{}", CTR.fetch_add(1, Ordering::Relaxed))
}

#[component]
pub fn TogglePrimitive(
    children: Children,
    #[prop(default = ToggleState::Off)] pressed: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    let t = toggle_attrs(pressed);
    view! {
        <label
            data-rs-toggle=""
            data-rs-uid=toggle_uid()
            data-rs-interaction="init"
            data-rs-component="Toggle"
            data-rs-behavior="toggle"
            data-rs-state=t.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            aria-label=if aria_label.is_empty() { None } else { Some(aria_label) }
            aria-disabled=d.aria_disabled
            class=class
        >
            <input
                type="checkbox"
                data-rs-toggle-input=""
                checked=t.data_rs_state == "on"
                tabindex="-1"
            />
            <span data-rs-toggle-content="">
                {children()}
            </span>
        </label>
    }
}
