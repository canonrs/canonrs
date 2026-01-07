//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct CollapsibleContext {
    pub open: RwSignal<bool>,
}

#[component]
pub fn CollapsiblePrimitive(
    children: ChildrenFn,
    #[prop(into)] open: RwSignal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_open_change: Callback<bool>,
) -> impl IntoView {
    let ctx = CollapsibleContext { open };
    provide_context(ctx);

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        on_open_change.run(open.get());
    });

    view! {
        <div data-slot="collapsible">
            {children()}
        </div>
    }
}

#[component]
pub fn CollapsibleTriggerPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleContext>();
    
    view! {
        <button
            class=class
            on:click=move |_| ctx.open.update(|o| *o = !*o)
            data-slot="collapsible-trigger"
            aria-expanded=move || if ctx.open.get() { "true" } else { "false" }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CollapsibleContentPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let ctx = expect_context::<CollapsibleContext>();
    let classes = StoredValue::new(class);
    
    view! {
        <Show when=move || ctx.open.get()>
            <div
                class=classes.get_value()
                data-slot="collapsible-content"
            >
                {children()}
            </div>
        </Show>
    }
}
