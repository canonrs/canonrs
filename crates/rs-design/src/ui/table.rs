use crate::primitives::table::*;
use leptos::prelude::*;

#[component]
pub fn Table(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TablePrimitive>
            {children()}
        </TablePrimitive>
    }
}

#[component]
pub fn TableHeader(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableHeaderPrimitive>
            {children()}
        </TableHeaderPrimitive>
    }
}

#[component]
pub fn TableBody(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableBodyPrimitive>
            {children()}
        </TableBodyPrimitive>
    }
}

#[component]
pub fn TableFooter(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableFooterPrimitive>
            {children()}
        </TableFooterPrimitive>
    }
}

#[component]
pub fn TableRow(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableRowPrimitive>
            {children()}
        </TableRowPrimitive>
    }
}

#[component]
pub fn TableHead(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableHeadPrimitive>
            {children()}
        </TableHeadPrimitive>
    }
}

#[component]
pub fn TableCell(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableCellPrimitive>
            {children()}
        </TableCellPrimitive>
    }
}

#[component]
pub fn TableCaption(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TableCaptionPrimitive>
            {children()}
        </TableCaptionPrimitive>
    }
}
