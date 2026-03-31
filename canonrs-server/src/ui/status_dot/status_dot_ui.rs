
use leptos::prelude::*;
use canonrs_core::primitives::StatusDotPrimitive;
pub use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDot(
    children: Children,
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatusDotPrimitive variant=variant class=class>
            {children()}
        </StatusDotPrimitive>
    }
}

#[component]
pub fn StatusDotPreview() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:0.75rem;">
            <StatusDot variant=StatusDotVariant::Online>"Online"</StatusDot>
            <StatusDot variant=StatusDotVariant::Offline>"Offline"</StatusDot>
            <StatusDot variant=StatusDotVariant::Away>"Away"</StatusDot>
            <StatusDot variant=StatusDotVariant::Busy>"Busy"</StatusDot>
            <StatusDot variant=StatusDotVariant::DoNotDisturb>"DND"</StatusDot>
        </div>
    }
}
