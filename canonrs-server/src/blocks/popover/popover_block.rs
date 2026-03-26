//! # Popover Block
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
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="popover"
            data-block-version="1"
            data-block-placement=placement.as_str()
            class=class
        >
            {trigger.map(|t| view! { <div data-block-region="trigger">{t()}</div> })}
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
