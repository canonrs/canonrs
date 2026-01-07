//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn PopoverPrimitive(
    #[prop(default = false)] open: bool,
    #[prop(default = Callback::new(|_| {}))] on_change: Callback<bool>,
    children: Children,
) -> impl IntoView {
    let is_open = RwSignal::new(open);

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        on_change.run(is_open.get());
    });

    provide_context(PopoverContext { is_open });

    view! {
        <div data-slot="popover" data-state=move || if is_open.get() { "open" } else { "closed" }>
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverTriggerPrimitive(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    view! {
        <div
            on:click=move |_| ctx.is_open.update(|open| *open = !*open)
            data-slot="popover-trigger"
            aria-haspopup="dialog"
            aria-expanded=move || ctx.is_open.get()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverContentPrimitive(
    #[prop(default = "center".to_string(), into)] align: String,
    #[prop(default = 4)] side_offset: i32,
    #[prop(default = String::new(), into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    view! {
        <Show when=move || ctx.is_open.get()>
            <div
                role="dialog"
                data-slot="popover-content"
                data-state="open"
                data-align=align.clone()
                style=format!("--side-offset: {}px", side_offset)
                class=class.clone()
            >
                {children()}
            </div>
        </Show>
    }
}

#[derive(Clone, Copy)]
struct PopoverContext {
    is_open: RwSignal<bool>,
}
