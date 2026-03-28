//! @canon-id: popover
//! @canon-type: block
//! @canon-category: overlay
//! @canon-variant: overlay
//! @canon-container: true
//! @canon-regions: trigger, header, content, footer
//! @canon-label: Popover
//! @canon-description: Floating popover container block
//! @canon-tags: popover, floating, tooltip, overlay, context
//! @canon-slot-accepts: trigger=Any,header=Any,content=Any,footer=Action
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum PopoverPlacement { #[default] Bottom, Top, Left, Right }
impl PopoverPlacement {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Bottom => "bottom", Self::Top => "top", Self::Left => "left", Self::Right => "right" }
    }
}

#[component]
pub fn PopoverBlock(
    #[prop(default = PopoverPlacement::Bottom)] placement: PopoverPlacement,
    #[prop(optional)] trigger: Option<ChildrenFn>,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="Popover"
            data-rs-placement=placement.as_str()
            style=style
            class=class
        >
            {trigger.map(|t| view! { <div data-rs-region="trigger">{t()}</div> })}
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
