//! @canon-level: strict
//! Banner Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::banner_ui::{
    Banner as BannerUi,
    BannerClose,
    BannerContent
};
pub use canonrs_core::primitives::BannerVariant;

#[component]
pub fn Banner(
    #[prop(into, optional)] content: Option<String>,
    #[prop(default = BannerVariant::Info)] variant: BannerVariant,
    #[prop(default = true)] dismissible: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <BannerUi variant=variant class=class>
            <BannerContent>{content}</BannerContent>
            {dismissible.then(|| view! { <BannerClose>"×"</BannerClose> })}
        </BannerUi>
    }
}
