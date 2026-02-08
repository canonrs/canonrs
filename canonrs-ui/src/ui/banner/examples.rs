use leptos::prelude::*;
use super::{Banner, BannerContent, BannerActions, BannerClose, BannerVariant, BannerPosition};

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Banner variant=BannerVariant::Info>
            <BannerContent>
                "This is an informational banner message."
            </BannerContent>
            <BannerActions>
                <button>"Action"</button>
            </BannerActions>
            <BannerClose>
                "×"
            </BannerClose>
        </Banner>
    }
}

#[component]
pub fn variants_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Banner variant=BannerVariant::Info>
                <BannerContent>"Info banner"</BannerContent>
            </Banner>
            <Banner variant=BannerVariant::Success>
                <BannerContent>"Success banner"</BannerContent>
            </Banner>
            <Banner variant=BannerVariant::Warning>
                <BannerContent>"Warning banner"</BannerContent>
            </Banner>
            <Banner variant=BannerVariant::Error>
                <BannerContent>"Error banner"</BannerContent>
            </Banner>
        </div>
    }
}

#[component]
pub fn positions_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Banner position=BannerPosition::Top>
                <BannerContent>"Top positioned banner"</BannerContent>
            </Banner>
            <Banner position=BannerPosition::Bottom>
                <BannerContent>"Bottom positioned banner"</BannerContent>
            </Banner>
        </div>
    }
}
