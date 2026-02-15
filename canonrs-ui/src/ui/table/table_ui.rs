use leptos::prelude::*;
use crate::primitives::{
    TableWrapperPrimitive,
    TablePrimitive,
    TableHeaderPrimitive,
    TableBodyPrimitive,
    TableFooterPrimitive,
    TableRowPrimitive,
    TableHeadPrimitive,
    TableCellPrimitive,
    TableCaptionPrimitive,
};

#[component]
pub fn Table(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] striped: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableWrapperPrimitive class=class id=id>
            <TablePrimitive striped=striped>
                {children.map(|c| c())}
            </TablePrimitive>
        </TableWrapperPrimitive>
    }
}

#[component]
pub fn TableHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableHeaderPrimitive class=class id=id>
            {children.map(|c| c())}
        </TableHeaderPrimitive>
    }
}

#[component]
pub fn TableBody(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableBodyPrimitive class=class id=id>
            {children.map(|c| c())}
        </TableBodyPrimitive>
    }
}

#[component]
pub fn TableFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableFooterPrimitive class=class id=id>
            {children.map(|c| c())}
        </TableFooterPrimitive>
    }
}

#[component]
pub fn TableRow(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableRowPrimitive selected=selected class=class id=id>
            {children.map(|c| c())}
        </TableRowPrimitive>
    }
}

#[component]
pub fn TableHead(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableHeadPrimitive class=class id=id>
            {children.map(|c| c())}
        </TableHeadPrimitive>
    }
}

#[component]
pub fn TableCell(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableCellPrimitive class=class id=id>
            {children.map(|c| c())}
        </TableCellPrimitive>
    }
}

#[component]
pub fn TableCaption(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <TableCaptionPrimitive class=class id=id>
            {children.map(|c| c())}
        </TableCaptionPrimitive>
    }
}
