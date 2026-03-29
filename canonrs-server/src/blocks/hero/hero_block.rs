//! @canon-id: hero
//! @canon-type: block
//! @canon-category: page
//! @canon-container: true
//! @canon-regions: header, media, content, actions, footer
//! @canon-label: Hero
//! @canon-description: Page hero block with media, content and actions regions
//! @canon-tags: hero, landing, intro, cta, media, page
//! @canon-slot-accepts: header=Any,media=Any,content=Any,actions=Action,footer=Any
//! @canon-prop: variant | Select(centered:Centered,split:Split,media-top:MediaTop) | centered | visual | variant
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum HeroVariant {
    #[default]
    Centered,
    Split,
    MediaTop,
}

impl HeroVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Centered => "centered",
            Self::Split    => "split",
            Self::MediaTop => "media-top",
        }
    }
}

#[component]
pub fn Hero(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] media: Option<ChildrenFn>,
    content: ChildrenFn,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = HeroVariant::Centered)] variant: HeroVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="Hero"
            data-rs-behavior="structural"
            data-rs-variant=variant.as_str()
            class=class
        >
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {media.map(|m| view! { <div data-rs-region="media">{m()}</div> })}
            <div data-rs-region="content">{content()}</div>
            {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
