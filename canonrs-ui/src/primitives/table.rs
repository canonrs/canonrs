//! @canon-level: strict
//! @canon-owner: primitives-team
//! Table Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] striped: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <table 
            data-table="" 
            attr:data-striped={striped.then(|| "true")}
            class=class 
            id=id
        >
            {children.map(|c| c())}
        </table>
    }
}

#[component]
pub fn TableHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <thead data-table-header="" class=class id=id>
            {children.map(|c| c())}
        </thead>
    }
}

#[component]
pub fn TableBodyPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tbody data-table-body="" class=class id=id>
            {children.map(|c| c())}
        </tbody>
    }
}

#[component]
pub fn TableFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tfoot data-table-footer="" class=class id=id>
            {children.map(|c| c())}
        </tfoot>
    }
}

#[component]
pub fn TableRowPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tr 
            data-table-row="" 
            attr:data-state={selected.then(|| "selected")}
            class=class 
            id=id
        >
            {children.map(|c| c())}
        </tr>
    }
}

#[component]
pub fn TableHeadPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <th data-table-head="" scope="col" class=class id=id>
            {children.map(|c| c())}
        </th>
    }
}

#[component]
pub fn TableCellPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <td data-table-cell="" class=class id=id>
            {children.map(|c| c())}
        </td>
    }
}

#[component]
pub fn TableCaptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <caption data-table-caption="" class=class id=id>
            {children.map(|c| c())}
        </caption>
    }
}
