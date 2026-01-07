//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct DialogContext {
    pub open: RwSignal<bool>,
}

#[component]
pub fn DialogPrimitive(
    children: ChildrenFn,
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_open_change: Callback<bool>,
) -> impl IntoView {
    let ctx = DialogContext { open };
    provide_context(ctx);

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        on_open_change.run(open.get());
    });

    view! {
        <div data-dialog="">
            {children()}
        </div>
    }
}

#[component]
pub fn DialogTriggerPrimitive(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <button
            class=classes.get_value()
            on:click=move |_| ctx.open.set(true)
            data-dialog-trigger=""
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DialogBackdropPrimitive(
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <Show when=move || ctx.open.get()>
            <div
                class=classes.get_value()
                on:click=move |_| ctx.open.set(false)
                data-dialog-backdrop=""
            />
        </Show>
    }
}

#[component]
pub fn DialogPopupPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <Show when=move || ctx.open.get()>
            <div
                class=classes.get_value()
                data-dialog-popup=""
                role="dialog"
                aria-modal="true"
            >
                {children()}
            </div>
        </Show>
    }
}

#[component]
pub fn DialogClosePrimitive(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <button
            class=classes.get_value()
            on:click=move |_| ctx.open.set(false)
            data-dialog-close=""
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DialogTitlePrimitive(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(class);
    view! {
        <h2 class=classes.get_value() data-dialog-title="">
            {children()}
        </h2>
    }
}

#[component]
pub fn DialogDescriptionPrimitive(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let classes = StoredValue::new(class);
    view! {
        <p class=classes.get_value() data-dialog-description="">
            {children()}
        </p>
    }
}
