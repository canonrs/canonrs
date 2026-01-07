use crate::primitives::sheet::*;
use leptos::prelude::*;

#[component]
pub fn Sheet(
    children: ChildrenFn,
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_open_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <SheetPrimitive open=open on_open_change=on_open_change>
            {children()}
        </SheetPrimitive>
    }
}

#[component]
pub fn SheetTrigger(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SheetTriggerPrimitive class=class>
            {children()}
        </SheetTriggerPrimitive>
    }
}

#[component]
pub fn SheetClose(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SheetClosePrimitive class=class>
            {children()}
        </SheetClosePrimitive>
    }
}

#[component]
pub fn SheetContent(
    children: ChildrenFn,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SheetContentPrimitive side=side class=class>
            {children()}
        </SheetContentPrimitive>
    }
}

#[component]
pub fn SheetHeader(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("flex flex-col gap-1.5 p-4 {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetFooter(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("mt-auto flex flex-col gap-2 p-4 {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn SheetTitle(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SheetTitlePrimitive class=format!("font-semibold text-foreground {}", class)>
            {children()}
        </SheetTitlePrimitive>
    }
}

#[component]
pub fn SheetDescription(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <SheetDescriptionPrimitive class=format!("text-sm text-muted-foreground {}", class)>
            {children()}
        </SheetDescriptionPrimitive>
    }
}
