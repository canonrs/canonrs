//! @canon-level: strict
//! @canon-owner: primitives-team
//! EmptyTable Primitive - Empty state for tables

use leptos::prelude::*;

#[component]
pub fn EmptyTablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 999)] colspan: i32,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tr data-empty-table-row="" role="row" class=class id=id>
            <td colspan=colspan role="cell">
                <div data-empty-table-content="" role="status" aria-live="polite">
                    {children.map(|c| c())}
                </div>
            </td>
        </tr>
    }
}

#[component]
pub fn EmptyTableTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-empty-table-title="">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn EmptyTableDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div data-empty-table-description="">
            {children.map(|c| c())}
        </div>
    }
}
