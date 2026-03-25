//! @canon-level: strict
//! @canon-owner: primitives-team
//! EmptyTable Primitive - Empty state for tables

use leptos::prelude::*;

#[component]
pub fn EmptyTablePrimitive(
    children: Children,
    #[prop(default = 999)] colspan: i32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <tr data-rs-empty-table-row="" role="row" class=class>
            <td colspan=colspan role="cell">
                <div data-rs-empty-table-content="" role="status" aria-live="polite">
                    {children()}
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn EmptyTableTitlePrimitive(
    children: Children,
) -> impl IntoView {
    view! {
        <div data-rs-empty-table-title="">
            {children()}
        </div>
    }
}

#[component]
pub fn EmptyTableDescriptionPrimitive(
    children: Children,
) -> impl IntoView {
    view! {
        <div data-rs-empty-table-description="">
            {children()}
        </div>
    }
}
