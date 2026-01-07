//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive (not public API)
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct TooltipContext {
    pub open: RwSignal<bool>,
}

#[component]
pub fn TooltipProviderPrimitive(
    children: ChildrenFn,
    #[prop(default = 0)] delay_duration: u32,
) -> impl IntoView {
    // Provider não precisa de state em Leptos (CSS hover)
    view! {
        <div data-slot="tooltip-provider" data-delay={delay_duration}>
            {children()}
        </div>
    }
}

#[component]
pub fn TooltipPrimitive(
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let ctx = TooltipContext { open };
    provide_context(ctx);

    view! {
        <TooltipProviderPrimitive>
            <div 
                data-slot="tooltip"
                on:mouseenter=move |_| open.set(true)
                on:mouseleave=move |_| open.set(false)
            >
                {children()}
            </div>
        </TooltipProviderPrimitive>
    }
}

#[component]
pub fn TooltipTriggerPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div class=class data-slot="tooltip-trigger">
            {children()}
        </div>
    }
}

#[component]
pub fn TooltipContentPrimitive(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = 0)] side_offset: i32,
) -> impl IntoView {
    let ctx = expect_context::<TooltipContext>();
    let classes = StoredValue::new(class);

    view! {
        // Portal simulado (renderiza in-place, CSS position fixed)
        <Show when=move || ctx.open.get()>
            <div
                data-slot="tooltip-content"
                class=classes.get_value()
                style=format!("margin-top: {}px;", side_offset)
            >
                {children()}
                // Arrow
                <div 
                    class="absolute size-2.5 rotate-45 rounded-[2px] bg-foreground"
                    style="top: -5px; left: 50%; transform: translateX(-50%);"
                />
            </div>
        </Show>
    }
}
