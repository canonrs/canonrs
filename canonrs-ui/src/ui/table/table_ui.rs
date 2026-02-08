use leptos::prelude::*;
use crate::primitives::table::{
    TablePrimitive,
    TableHeaderPrimitive,
    TableBodyPrimitive,
    TableRowPrimitive,
    TableHeadPrimitive,
    TableCellPrimitive,
    TableFooterPrimitive,
    TableCaptionPrimitive,
};

#[component]
pub fn Table(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TablePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </TablePrimitive>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableHeaderPrimitive class={class}>
            {children.map(|c| c())}
        </TableHeaderPrimitive>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableBodyPrimitive class={class}>
            {children.map(|c| c())}
        </TableBodyPrimitive>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TableRowPrimitive class={class} id={id}>
            {children.map(|c| c())}
        </TableRowPrimitive>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableHeadPrimitive class={class}>
            {children.map(|c| c())}
        </TableHeadPrimitive>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableCellPrimitive class={class}>
            {children.map(|c| c())}
        </TableCellPrimitive>
    }
}

#[component]
pub fn TableFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableFooterPrimitive class={class}>
            {children.map(|c| c())}
        </TableFooterPrimitive>
    }
}

#[component]
pub fn TableCaption(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TableCaptionPrimitive class={class}>
            {children.map(|c| c())}
        </TableCaptionPrimitive>
    }
}
