use crate::primitives::tooltip::*;
use leptos::prelude::*;

#[component]
pub fn TooltipProvider(
    children: ChildrenFn,
    #[prop(default = 0)] delay_duration: u32,
) -> impl IntoView {
    view! {
        <TooltipProviderPrimitive delay_duration=delay_duration>
            {children()}
        </TooltipProviderPrimitive>
    }
}

#[component]
pub fn Tooltip(children: ChildrenFn) -> impl IntoView {
    view! {
        <TooltipPrimitive>
            {children()}
        </TooltipPrimitive>
    }
}

#[component]
pub fn TooltipTrigger(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <TooltipTriggerPrimitive class=class>
            {children()}
        </TooltipTriggerPrimitive>
    }
}

#[component]
pub fn TooltipContent(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = 0)] side_offset: i32,
) -> impl IntoView {
    let classes = format!(
        "z-50 w-fit rounded-md bg-foreground px-3 py-1.5 text-xs text-background {}",
        class
    );

    view! {
        <TooltipContentPrimitive class=classes side_offset=side_offset>
            {children()}
        </TooltipContentPrimitive>
    }
}
