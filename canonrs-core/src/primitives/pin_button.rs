//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pin Button Primitive - Button to pin/unpin columns

use leptos::prelude::*;

#[component]
pub fn PinButtonPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = String::new())] col_id: String,
    #[prop(default = false)] is_pinned: bool,
) -> impl IntoView {
    let uid_pb = crate::infra::uid::generate("pb");
    // data-pin-state cycles: unpinned → pinned-left → pinned-right → unpinned
    let initial_state = if is_pinned { "pinned-left" } else { "unpinned" };
    let initial_icon  = if is_pinned { "⬅📌" } else { "📍" };

    view! {
        <button
            data-rs-pin-button=""
            data-rs-uid=uid_pb
            data-pin-col-id={if col_id.is_empty() { None } else { Some(col_id) }}
            data-pin-state=initial_state
            type="button"
            class={class}
            title="Pin column"
            style="background: none; border: none; padding: 4px; cursor: pointer; display: flex; align-items: center; font-size: 0.75rem; gap: 2px;"
        >
            {initial_icon}
            {children()}
        </button>
    }
}
