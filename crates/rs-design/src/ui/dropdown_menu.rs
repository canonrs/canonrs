/// Canon Type: Type 3 - Interactive (Controlled State)
/// Families: A (Overlay & Layering), B (Selection & Lists)
use leptos::prelude::*;
use crate::primitives::dropdown_menu::*;

#[component]
pub fn DropdownMenu(
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive>
            {children()}
        </DropdownMenuPrimitive>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <DropdownMenuTriggerPrimitive class=class>
            {children()}
        </DropdownMenuTriggerPrimitive>
    }
}

#[component]
pub fn DropdownMenuContent(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = format!(
        "z-[var(--z-dropdown)] \
         min-w-[8rem] \
         origin-top-right \
         overflow-hidden \
         rounded-[var(--radius-md)] \
         border border-border \
         bg-card \
         p-[var(--space-xs)] \
         shadow-[var(--shadow-lg)] \
         {}",
        class
    );

    view! {
        <DropdownMenuContentPrimitive class=classes>
            {children()}
        </DropdownMenuContentPrimitive>
    }
}

#[component]
pub fn DropdownMenuLabel(
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = format!(
        "px-[var(--space-sm)] \
         py-[var(--space-xs)] \
         text-[length:var(--font-size-sm)] \
         font-[var(--font-weight-semibold)] \
         text-muted-foreground \
         {}",
        class
    );

    view! {
        <DropdownMenuLabelPrimitive class=classes>
            {children()}
        </DropdownMenuLabelPrimitive>
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let classes = format!(
        "-mx-[var(--space-xs)] \
         my-[var(--space-xs)] \
         h-[var(--border-width-hairline)] \
         bg-border \
         {}",
        class
    );

    view! {
        <DropdownMenuSeparatorPrimitive class=classes />
    }
}

#[component]
pub fn DropdownMenuItem(
    #[prop(optional, into)] class: String,
    #[prop(optional)] on_select: Option<Callback<()>>,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = format!(
        "relative flex cursor-pointer items-center gap-[var(--space-sm)] \
         rounded-[var(--radius-sm)] \
         px-[var(--space-sm)] \
         py-[var(--space-xs)] \
         text-[length:var(--font-size-sm)] \
         outline-hidden select-none \
         transition-colors duration-[var(--motion-duration-fast)] \
         hover:bg-muted \
         hover:text-foreground \
         focus:bg-muted \
         data-[disabled]:pointer-events-none \
         data-[disabled]:opacity-[var(--state-disabled-opacity)] \
         {}",
        class
    );

    let callback = on_select.unwrap_or_else(|| Callback::new(|_| {}));

    view! {
        <DropdownMenuItemPrimitive class=classes on_select=callback>
            {children()}
        </DropdownMenuItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = format!(
        "relative flex cursor-pointer items-center gap-[var(--space-sm)] \
         rounded-[var(--radius-sm)] \
         py-[var(--space-xs)] \
         pr-[var(--space-sm)] \
         pl-[calc(var(--space-sm)*3)] \
         text-[length:var(--font-size-sm)] \
         outline-hidden select-none \
         transition-colors duration-[var(--motion-duration-fast)] \
         hover:bg-muted \
         hover:text-foreground \
         focus:bg-muted \
         data-[disabled]:pointer-events-none \
         data-[disabled]:opacity-[var(--state-disabled-opacity)] \
         {}",
        class
    );

    view! {
        <DropdownMenuCheckboxItemPrimitive checked=checked class=classes>
            {children()}
        </DropdownMenuCheckboxItemPrimitive>
    }
}
