use leptos::prelude::*;
use super::{Banner, BannerContent, BannerClose, BannerVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    let (show_warning, set_show_warning) = signal(true);
    let (show_success, set_show_success) = signal(true);
    let (show_error, set_show_error) = signal(true);

    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            {move || show_warning.get().then(|| view! {
                <Banner variant=BannerVariant::Warning open=true>
                    <BannerContent>
                        <strong>"System Maintenance: "</strong>
                        "Scheduled downtime Saturday, Feb 15, 2AM-4AM EST."
                    </BannerContent>
                    <BannerClose on_close=Callback::new(move |_| set_show_warning.set(false))>
                        "×"
                    </BannerClose>
                </Banner>
            })}

            {move || show_success.get().then(|| view! {
                <Banner variant=BannerVariant::Success open=true>
                    <BannerContent>
                        <strong>"Deployment Complete: "</strong>
                        "Version 2.4.0 is now live with improved performance."
                    </BannerContent>
                    <BannerClose on_close=Callback::new(move |_| set_show_success.set(false))>
                        "×"
                    </BannerClose>
                </Banner>
            })}

            {move || show_error.get().then(|| view! {
                <Banner variant=BannerVariant::Error open=true>
                    <BannerContent>
                        <strong>"API Rate Limit: "</strong>
                        "You've reached 95% of your monthly quota."
                    </BannerContent>
                    <BannerClose on_close=Callback::new(move |_| set_show_error.set(false))>
                        "×"
                    </BannerClose>
                </Banner>
            })}
        </div>
    }
}
