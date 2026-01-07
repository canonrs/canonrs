use crate::primitives::dialog::*;
use crate::ui::button::{ButtonVariant, ButtonType};
use crate::ui::Button;
use leptos::prelude::*;

/// Dialog UI - Canon Type 3 (Interactive)
/// 
/// Shadcn-style dialog with Tailwind classes

// ═══════════════════════════════════════════════════════════
// ROOT & TRIGGER
// ═══════════════════════════════════════════════════════════

#[component]
pub fn Dialog(
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
pub fn DialogTrigger(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive class=class>
            {children()}
        </DialogTriggerPrimitive>
    }
}

// ═══════════════════════════════════════════════════════════
// OVERLAY (Backdrop)
// ═══════════════════════════════════════════════════════════

#[component]
pub fn DialogOverlay(
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(format!(
        "fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 {}",
        class
    ));
    
    view! {
        <DialogBackdropPrimitive class=classes.get_value() />
    }
}

// ═══════════════════════════════════════════════════════════
// CONTENT
// ═══════════════════════════════════════════════════════════

#[component]
pub fn DialogContent(
    children: ChildrenFn,
    #[prop(default = true)] show_close_button: bool,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(format!(
        "fixed top-[50%] left-[50%] z-50 w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] sm:rounded-lg {}",
        class
    ));
    
    view! {
        <DialogOverlay />
        <DialogPopupPrimitive class=classes.get_value()>
            {children()}
            <Show when=move || show_close_button>
                <DialogClose class="absolute top-4 right-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 15 15" fill="none" class="h-4 w-4">
                        <path d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"/>
                    </svg>
                    <span class="sr-only">"Close"</span>
                </DialogClose>
            </Show>
        </DialogPopupPrimitive>
    }
}

// ═══════════════════════════════════════════════════════════
// HEADER / FOOTER / TITLE / DESCRIPTION
// ═══════════════════════════════════════════════════════════

#[component]
pub fn DialogHeader(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(format!("flex flex-col space-y-1.5 text-center sm:text-left {}", class));
    
    view! {
        <div class=classes.get_value()>
            {children()}
        </div>
    }
}

#[component]
pub fn DialogFooter(
    children: ChildrenFn,
    #[prop(default = false)] show_close_button: bool,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(format!(
        "flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 {}",
        class
    ));
    
    view! {
        <div class=classes.get_value()>
            {children()}
            <Show when=move || show_close_button>
                <DialogClose class="">
                    <Button variant=ButtonVariant::Outline button_type=ButtonType::Button>
                        "Close"
                    </Button>
                </DialogClose>
            </Show>
        </div>
    }
}

#[component]
pub fn DialogTitle(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(format!("text-lg font-semibold leading-none tracking-tight {}", class));
    
    view! {
        <DialogTitlePrimitive class=classes.get_value()>
            {children()}
        </DialogTitlePrimitive>
    }
}

#[component]
pub fn DialogDescription(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(format!("text-sm text-muted-foreground {}", class));
    
    view! {
        <DialogDescriptionPrimitive class=classes.get_value()>
            {children()}
        </DialogDescriptionPrimitive>
    }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <DialogClosePrimitive class=class>
            {children()}
        </DialogClosePrimitive>
    }
}
