//! @canon-level: strict
//! @canon-owner: primitives-team
//! Table Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TableWrapperPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-table-wrapper="" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </div>
    }
}

#[component]
pub fn TablePrimitive(
    children: Children,
    #[prop(default = false)] striped: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <table 
            data-rs-table="" 
            attr:data-striped={striped.then_some("true")}
            class=class 
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </table>
    }
}

#[component]
pub fn TableHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <thead data-rs-table-header="" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBodyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tbody data-rs-table-body="" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tfoot data-rs-table-footer="" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </tfoot>
    }
}

#[component]
pub fn TableRowPrimitive(
    children: Children,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <tr 
            data-rs-table-row="" 
            data-rs-state={selected.then_some("selected")}
            class=class 
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHeadPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <th data-rs-table-head="" scope="col" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCellPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <td data-rs-table-cell="" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </td>
    }
}

#[component]
pub fn TableCaptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <caption data-rs-table-caption="" class=class id=if id.is_empty() { None } else { Some(id.clone()) }>
            {children()}
        </caption>
    }
}
