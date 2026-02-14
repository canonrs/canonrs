use leptos::prelude::*;
use crate::primitives::{BannerPrimitive, BannerClosePrimitive, BannerContentPrimitive, BannerActionsPrimitive};

pub use crate::primitives::BannerVariant;

#[component]
pub fn Banner(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = true)] open: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <BannerPrimitive variant=variant open=open class=class id=id>
            {children.map(|c| c())}
        </BannerPrimitive>
    }
}

#[component]
pub fn BannerContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <BannerContentPrimitive class=class id=id>
            {children.map(|c| c())}
        </BannerContentPrimitive>
    }
}

#[component]
pub fn BannerActions(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <BannerActionsPrimitive class=class id=id>
            {children.map(|c| c())}
        </BannerActionsPrimitive>
    }
}

#[component]
pub fn BannerClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    let callback = on_close.unwrap_or_else(|| Callback::new(|_| {}));
    
    view! {
        <BannerClosePrimitive on_close=callback class=class id=id>
            {children.map(|c| c())}
        </BannerClosePrimitive>
    }
}
