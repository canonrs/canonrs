//! @canon-level: strict
//! @canon-owner: primitives-team
//! Table Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TableWrapperPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-table-wrapper="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] striped: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <table 
            data-rs-table="" 
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
        <thead data-rs-table-header="" class=class id=id>
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
        <tbody data-rs-table-body="" class=class id=id>
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
        <tfoot data-rs-table-footer="" class=class id=id>
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
            data-rs-table-row="" 
            data-rs-state={selected.then(|| "selected")}
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
        <th data-rs-table-head="" scope="col" class=class id=id>
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
        <td data-rs-table-cell="" class=class id=id>
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
        <caption data-rs-table-caption="" class=class id=id>
            {children.map(|c| c())}
        </caption>
    }
}
