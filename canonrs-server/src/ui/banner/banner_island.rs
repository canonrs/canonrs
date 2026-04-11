//! @canon-level: strict
//! Banner Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::banner_ui::{Banner, BannerClose, BannerContent};
use canonrs_core::primitives::BannerVariant;

#[component]
pub fn BannerIsland(
    #[prop(into, optional)] content: Option<String>,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = true)] dismissible: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Banner variant=variant class=class>
            <BannerContent>{content}</BannerContent>
            {dismissible.then(|| view! { <BannerClose>"×"</BannerClose> })}
        </Banner>
    }
}
