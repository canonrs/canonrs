use leptos::prelude::*;
use super::stat_island::{
    StatIsland, StatHeaderIsland, StatBodyIsland,
    StatValueIsland, StatLabelIsland, StatDeltaIsland, StatIconIsland,
};
use super::stat_ui::{StatSize, StatAlign, StatTrend};

#[component]
pub fn StatShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <StatIsland size=StatSize::Lg trend=StatTrend::Increase>
                    <StatHeaderIsland>
                        <StatIconIsland>"📈"</StatIconIsland>
                        <StatLabelIsland>"Total Revenue"</StatLabelIsland>
                    </StatHeaderIsland>
                    <StatBodyIsland>
                        <StatValueIsland>"$89,432"</StatValueIsland>
                        <StatDeltaIsland>"+18.2%"</StatDeltaIsland>
                    </StatBodyIsland>
                </StatIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Metric layout and semantics enforced via structured primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Trend variants"</span>
                <div data-rs-showcase-preview-row="">
                    <StatIsland trend=StatTrend::Increase>
                        <StatLabelIsland>"Active Users"</StatLabelIsland>
                        <StatBodyIsland>
                            <StatValueIsland>"2,350"</StatValueIsland>
                            <StatDeltaIsland>"+12%"</StatDeltaIsland>
                        </StatBodyIsland>
                    </StatIsland>
                    <StatIsland trend=StatTrend::Decrease>
                        <StatLabelIsland>"Bounce Rate"</StatLabelIsland>
                        <StatBodyIsland>
                            <StatValueIsland>"3.2%"</StatValueIsland>
                            <StatDeltaIsland>"-0.5%"</StatDeltaIsland>
                        </StatBodyIsland>
                    </StatIsland>
                    <StatIsland trend=StatTrend::Neutral>
                        <StatLabelIsland>"Sessions"</StatLabelIsland>
                        <StatBodyIsland>
                            <StatValueIsland>"1,024"</StatValueIsland>
                            <StatDeltaIsland>"0%"</StatDeltaIsland>
                        </StatBodyIsland>
                    </StatIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Size variants"</span>
                <div data-rs-showcase-preview-row="">
                    <StatIsland size=StatSize::Sm>
                        <StatLabelIsland>"Small"</StatLabelIsland>
                        <StatValueIsland>"123"</StatValueIsland>
                    </StatIsland>
                    <StatIsland size=StatSize::Md>
                        <StatLabelIsland>"Medium"</StatLabelIsland>
                        <StatValueIsland>"4,567"</StatValueIsland>
                    </StatIsland>
                    <StatIsland size=StatSize::Lg>
                        <StatLabelIsland>"Large"</StatLabelIsland>
                        <StatValueIsland>"89,432"</StatValueIsland>
                    </StatIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Align + Icon"</span>
                <div data-rs-showcase-preview-row="">
                    <StatIsland align=StatAlign::Center>
                        <StatLabelIsland>"Centered"</StatLabelIsland>
                        <StatValueIsland>"999"</StatValueIsland>
                    </StatIsland>
                    <StatIsland>
                        <StatHeaderIsland>
                            <StatIconIsland>"💰"</StatIconIsland>
                            <StatLabelIsland>"Sales"</StatLabelIsland>
                        </StatHeaderIsland>
                        <StatValueIsland>"$12,234"</StatValueIsland>
                    </StatIsland>
                </div>
            </div>
        </div>
    }
}
