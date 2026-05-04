//! @canon-level: strict
//! @canon-owner: primitives-team
//! EmptyTable Primitive - Empty state for tables

use leptos::prelude::*;

#[component]
pub fn EmptyTablePrimitive(
    children: Children,
    #[prop(default = 1u32)] colspan: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_et = crate::infra::uid::generate("et");
    view! {
        <tr
            data-rs-empty-table-row=""
            data-rs-uid=uid_et
            role="row"
            class=class
        >
            <td colspan=colspan role="cell">
                <div
                    data-rs-empty-table-content=""
                    data-rs-activity="empty"
                    role="status"
                    aria-live="polite"
                >
                    {children()}
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn EmptyTableTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-empty-table-title="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyTableDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-empty-table-description="" class=class>
            {children()}
        </div>
    }
}
