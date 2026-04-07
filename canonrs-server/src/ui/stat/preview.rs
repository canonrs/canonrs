use leptos::prelude::*;
use super::stat_island::{
    StatIsland, StatHeaderIsland, StatBodyIsland,
    StatValueIsland, StatLabelIsland, StatDeltaIsland, StatIconIsland,
};

#[component]
pub fn StatShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <StatIsland size="lg" trend="increase">
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
                    <StatIsland trend="increase">
                        <StatLabelIsland>"Active Users"</StatLabelIsland>
                        <StatBodyIsland>
                            <StatValueIsland>"2,350"</StatValueIsland>
                            <StatDeltaIsland>"+12%"</StatDeltaIsland>
                        </StatBodyIsland>
                    </StatIsland>
                    <StatIsland trend="decrease">
                        <StatLabelIsland>"Bounce Rate"</StatLabelIsland>
                        <StatBodyIsland>
                            <StatValueIsland>"3.2%"</StatValueIsland>
                            <StatDeltaIsland>"-0.5%"</StatDeltaIsland>
                        </StatBodyIsland>
                    </StatIsland>
                    <StatIsland trend="neutral">
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
                    <StatIsland size="sm">
                        <StatLabelIsland>"Small"</StatLabelIsland>
                        <StatValueIsland>"123"</StatValueIsland>
                    </StatIsland>
                    <StatIsland size="md">
                        <StatLabelIsland>"Medium"</StatLabelIsland>
                        <StatValueIsland>"4,567"</StatValueIsland>
                    </StatIsland>
                    <StatIsland size="lg">
                        <StatLabelIsland>"Large"</StatLabelIsland>
                        <StatValueIsland>"89,432"</StatValueIsland>
                    </StatIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Align + Icon"</span>
                <div data-rs-showcase-preview-row="">
                    <StatIsland align="center">
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
