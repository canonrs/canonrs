//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct DropdownMenuContext {
    pub open: RwSignal<bool>,
}

#[component]
pub fn DropdownMenuPrimitive(
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let ctx = DropdownMenuContext { open };
    provide_context(ctx);

    view! {
        <div data-slot="dropdown-menu">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTriggerPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();
    
    view! {
        <button
            class=class
            on:click=move |_| ctx.open.update(|o| *o = !*o)
            data-slot="dropdown-menu-trigger"
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuContentPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = 4)] side_offset: i32,
) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <Show when=move || ctx.open.get()>
            <div
                class=classes.get_value()
                style=format!("margin-top: {}px;", side_offset)
                data-slot="dropdown-menu-content"
            >
                {children()}
            </div>
        </Show>
    }
}

#[component]
pub fn DropdownMenuGroupPrimitive(
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div data-slot="dropdown-menu-group">
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuItemPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = false)] inset: bool,
    #[prop(default = false)] destructive: bool,
    #[prop(default = Callback::new(|_| {}))] on_select: Callback<()>,
) -> impl IntoView {
    let ctx = expect_context::<DropdownMenuContext>();
    
    view! {
        <div
            class=class
            data-slot="dropdown-menu-item"
            data-inset=inset
            data-variant=if destructive { "destructive" } else { "default" }
            on:click=move |_| {
                on_select.run(());
                ctx.open.set(false);
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuCheckboxItemPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
    #[prop(into)] checked: RwSignal<bool>,
) -> impl IntoView {
    let classes = StoredValue::new(class);
    
    view! {
        <div
            class=classes.get_value()
            data-slot="dropdown-menu-checkbox-item"
            on:click=move |_| checked.update(|c| *c = !*c)
        >
            <span class="absolute left-2 flex size-3.5 items-center justify-center">
                <Show when=move || checked.get()>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" class="size-4">
                        <polyline points="20 6 9 17 4 12"/>
                    </svg>
                </Show>
            </span>
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuLabelPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = false)] inset: bool,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-slot="dropdown-menu-label"
            data-inset=inset
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparatorPrimitive(
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-slot="dropdown-menu-separator"
        />
    }
}
