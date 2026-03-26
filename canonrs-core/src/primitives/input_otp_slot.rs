//! @canon-level: strict
//! @canon-owner: primitives-team
//! InputOtpSlot Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::ActivityState;
use crate::state_engine::activity_attrs;

#[component]
pub fn InputOtpSlotPrimitive(
    children: Children,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let a = activity_attrs(state);
    view! {
        <div
            data-rs-input-otp-slot=""
            data-rs-state=a.data_rs_state
            aria-hidden="true"
            class=class
        >
            {children()}
        </div>
    }
}
