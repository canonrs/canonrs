//! @canon-id: drawer
//! @canon-type: block
//! @canon-category: overlay
//! @canon-variant: overlay
//! @canon-container: true
//! @canon-regions: header, content, footer
//! @canon-label: Drawer
//! @canon-description: Slide-out drawer panel block
//! @canon-tags: drawer,painel,slide,lateral,gaveta
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum DrawerPosition { Left, #[default] Right, Top, Bottom }
impl DrawerPosition {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Left => "left", Self::Right => "right", Self::Top => "top", Self::Bottom => "bottom" }
    }
}

#[component]
pub fn DrawerBlock(
    #[prop(default = DrawerPosition::Right)] position: DrawerPosition,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="drawer"
            data-block-version="1"
            style=style
            data-block-position=position.as_str()
            class=class
        >
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
