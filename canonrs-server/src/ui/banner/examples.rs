use leptos::prelude::*;
use super::{Banner, BannerContent, BannerClose, BannerVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display:flex;flex-direction:column;gap:1rem;">
            <Banner variant=BannerVariant::Warning>
                <BannerContent>
                    <strong>"System Maintenance: "</strong>
                    "Scheduled downtime Saturday, Feb 15, 2AM-4AM EST."
                </BannerContent>
                <BannerClose>"×"</BannerClose>
            </Banner>
            <Banner variant=BannerVariant::Success>
                <BannerContent>
                    <strong>"Deployment Complete: "</strong>
                    "Version 2.4.0 is now live with improved performance."
                </BannerContent>
                <BannerClose>"×"</BannerClose>
            </Banner>
            <Banner variant=BannerVariant::Error>
                <BannerContent>
                    <strong>"API Rate Limit: "</strong>
                    "You have reached 95% of your monthly quota."
                </BannerContent>
                <BannerClose>"×"</BannerClose>
            </Banner>
        </div>
    }
}
