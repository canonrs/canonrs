use leptos::prelude::*;
use crate::primitives::{
    AlertDialogPrimitive,
    AlertDialogTriggerPrimitive,
    AlertDialogOverlayPrimitive,
    AlertDialogContentPrimitive,
    AlertDialogTitlePrimitive,
    AlertDialogDescriptionPrimitive,
    AlertDialogPortalPrimitive,
    AlertDialogHeaderPrimitive,
    AlertDialogFooterPrimitive,
    AlertDialogActionPrimitive,
    AlertDialogCancelPrimitive,
};

#[component]
pub fn AlertDialog(
    children: ChildrenFn,
    #[prop(optional)] class: Option<String>,
    #[prop(into)] id: String,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    
    view! {
        <AlertDialogPrimitive
            class=class
            id=id.clone()
        >
            {children()}
        </AlertDialogPrimitive>
    }
}

#[component]
pub fn AlertDialogTrigger(
    children: Children,
    #[prop(into)] target_dialog_id: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogTriggerPrimitive
            target_dialog_id=target_dialog_id
            class=class
            id=id
        >
            {children()}
        </AlertDialogTriggerPrimitive>
    }
}

#[component]
pub fn AlertDialogPortal(
    children: Children,
) -> impl IntoView {
    view! {
        <AlertDialogPortalPrimitive>
            {children()}
        </AlertDialogPortalPrimitive>
    }
}

#[component]
pub fn AlertDialogOverlay(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogOverlayPrimitive
            class=class
            id=id
        />
    }
}

#[component]
pub fn AlertDialogContent(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogContentPrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogContentPrimitive>
    }
}

#[component]
pub fn AlertDialogHeader(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogHeaderPrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogHeaderPrimitive>
    }
}

#[component]
pub fn AlertDialogTitle(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogTitlePrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogTitlePrimitive>
    }
}

#[component]
pub fn AlertDialogDescription(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogDescriptionPrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogDescriptionPrimitive>
    }
}

#[component]
pub fn AlertDialogFooter(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogFooterPrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogFooterPrimitive>
    }
}

#[component]
pub fn AlertDialogAction(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogActionPrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogActionPrimitive>
    }
}

#[component]
pub fn AlertDialogCancel(
    children: Children,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let id = id.unwrap_or_default();
    
    view! {
        <AlertDialogCancelPrimitive
            class=class
            id=id
        >
            {children()}
        </AlertDialogCancelPrimitive>
    }
}
