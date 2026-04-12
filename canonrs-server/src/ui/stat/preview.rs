use leptos::prelude::*;
use super::stat_boundary::{
    Stat, StatHeader, StatBody,
    StatValue, StatLabel, StatDelta, StatIcon,
};
use super::stat_ui::{StatSize, StatAlign, StatTrend};

#[component]
pub fn StatShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Stat size=StatSize::Lg trend=StatTrend::Increase>
                    <StatHeader>
                        <StatIcon>"📈"</StatIcon>
                        <StatLabel>"Total Revenue"</StatLabel>
                    </StatHeader>
                    <StatBody>
                        <StatValue>"$89,432"</StatValue>
                        <StatDelta>"+18.2%"</StatDelta>
                    </StatBody>
                </Stat>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Metric layout and semantics enforced via structured primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Trend variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Stat trend=StatTrend::Increase>
                        <StatLabel>"Active Users"</StatLabel>
                        <StatBody>
                            <StatValue>"2,350"</StatValue>
                            <StatDelta>"+12%"</StatDelta>
                        </StatBody>
                    </Stat>
                    <Stat trend=StatTrend::Decrease>
                        <StatLabel>"Bounce Rate"</StatLabel>
                        <StatBody>
                            <StatValue>"3.2%"</StatValue>
                            <StatDelta>"-0.5%"</StatDelta>
                        </StatBody>
                    </Stat>
                    <Stat trend=StatTrend::Neutral>
                        <StatLabel>"Sessions"</StatLabel>
                        <StatBody>
                            <StatValue>"1,024"</StatValue>
                            <StatDelta>"0%"</StatDelta>
                        </StatBody>
                    </Stat>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Size variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Stat size=StatSize::Sm>
                        <StatLabel>"Small"</StatLabel>
                        <StatValue>"123"</StatValue>
                    </Stat>
                    <Stat size=StatSize::Md>
                        <StatLabel>"Medium"</StatLabel>
                        <StatValue>"4,567"</StatValue>
                    </Stat>
                    <Stat size=StatSize::Lg>
                        <StatLabel>"Large"</StatLabel>
                        <StatValue>"89,432"</StatValue>
                    </Stat>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Align + Icon"</span>
                <div data-rs-showcase-preview-row="">
                    <Stat align=StatAlign::Center>
                        <StatLabel>"Centered"</StatLabel>
                        <StatValue>"999"</StatValue>
                    </Stat>
                    <Stat>
                        <StatHeader>
                            <StatIcon>"💰"</StatIcon>
                            <StatLabel>"Sales"</StatLabel>
                        </StatHeader>
                        <StatValue>"$12,234"</StatValue>
                    </Stat>
                </div>
            </div>
        </div>
    }
}
