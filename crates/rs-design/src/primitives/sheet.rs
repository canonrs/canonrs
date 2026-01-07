//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

// Sheet é basicamente Dialog com animações de slide
pub use crate::primitives::dialog::{DialogContext, DialogPrimitive};

#[derive(Clone, Copy, PartialEq)]
pub enum SheetSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[component]
pub fn SheetPrimitive(
    children: ChildrenFn,
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_open_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open on_open_change=on_open_change>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn SheetTriggerPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    use crate::primitives::dialog::DialogTriggerPrimitive;
    view! {
        <DialogTriggerPrimitive class=class>
            {children()}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn SheetClosePrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    use crate::primitives::dialog::DialogClosePrimitive;
    view! {
        <DialogClosePrimitive class=class>
            {children()}
        </DialogClosePrimitive>
    }
}

#[component]
pub fn SheetOverlayPrimitive(
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    use crate::primitives::dialog::DialogBackdropPrimitive;
    view! {
        <DialogBackdropPrimitive class=class />
    }
}

#[component]
pub fn SheetContentPrimitive(
    children: ChildrenFn,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    use crate::primitives::dialog::DialogPopupPrimitive;
    let classes = StoredValue::new(class);
    
    let side_classes = match side {
        SheetSide::Right => "inset-y-0 right-0 h-full w-3/4 border-l sm:max-w-sm data-[state=open]:slide-in-from-right data-[state=closed]:slide-out-to-right",
        SheetSide::Left => "inset-y-0 left-0 h-full w-3/4 border-r sm:max-w-sm data-[state=open]:slide-in-from-left data-[state=closed]:slide-out-to-left",
        SheetSide::Top => "inset-x-0 top-0 h-auto border-b data-[state=open]:slide-in-from-top data-[state=closed]:slide-out-to-top",
        SheetSide::Bottom => "inset-x-0 bottom-0 h-auto border-t data-[state=open]:slide-in-from-bottom data-[state=closed]:slide-out-to-bottom",
    };
    
    let full_classes = format!(
        "fixed z-50 flex flex-col gap-4 bg-background shadow-lg transition ease-in-out duration-300 {}",
        side_classes
    );
    
    view! {
        <SheetOverlayPrimitive class="fixed inset-0 z-50 bg-black/50" />
        <DialogPopupPrimitive class=format!("{} {}", full_classes, classes.get_value())>
            {children()}
            <SheetClosePrimitive class="absolute top-4 right-4 rounded-sm opacity-70 hover:opacity-100">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="size-4">
                    <path d="M18 6 6 18M6 6l12 12"/>
                </svg>
                <span class="sr-only">"Close"</span>
            </SheetClosePrimitive>
        </DialogPopupPrimitive>
    }
}

#[component]
pub fn SheetTitlePrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    use crate::primitives::dialog::DialogTitlePrimitive;
    view! {
        <DialogTitlePrimitive class=class>
            {children()}
        </DialogTitlePrimitive>
    }
}

#[component]
pub fn SheetDescriptionPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    use crate::primitives::dialog::DialogDescriptionPrimitive;
    view! {
        <DialogDescriptionPrimitive class=class>
            {children()}
        </DialogDescriptionPrimitive>
    }
}
