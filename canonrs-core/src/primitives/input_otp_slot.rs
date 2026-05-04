//! @canon-level: strict
//! @canon-owner: primitives-team
//! InputOtpSlot Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::ActivityState;

#[component]
pub fn InputOtpSlotPrimitive(
    children: Children,
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_ios = crate::infra::uid::generate("ios");
    view! {
        <div
            data-rs-input-otp-slot=""
            data-rs-uid=uid_ios
            data-rs-activity=state.as_str()
            class=class
        >
            <span data-rs-slot-inner="">{children()}</span>
        </div>
    }
}
