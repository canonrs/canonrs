use leptos::prelude::*;
use super::{Stat, StatSize, StatTrend, StatAlign, StatHeader, StatBody, StatValue, StatLabel, StatDelta, StatIcon};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem;">
            // Basic stat
            <Stat>
                <StatLabel>"Total Revenue"</StatLabel>
                <StatValue>"$45,231"</StatValue>
            </Stat>

            // With trend up
            <Stat trend=StatTrend::Increase>
                <StatLabel>"Active Users"</StatLabel>
                <StatBody>
                    <StatValue>"2,350"</StatValue>
                    <StatDelta>"+12%"</StatDelta>
                </StatBody>
            </Stat>

            // With trend down
            <Stat trend=StatTrend::Decrease>
                <StatLabel>"Bounce Rate"</StatLabel>
                <StatBody>
                    <StatValue>"3.2%"</StatValue>
                    <StatDelta>"-0.5%"</StatDelta>
                </StatBody>
            </Stat>

            // With icon
            <Stat>
                <StatHeader>
                    <StatIcon>"💰"</StatIcon>
                    <StatLabel>"Sales"</StatLabel>
                </StatHeader>
                <StatValue>"$12,234"</StatValue>
            </Stat>

            // Size variants
            <Stat size=StatSize::Sm>
                <StatLabel>"Small"</StatLabel>
                <StatValue>"123"</StatValue>
            </Stat>

            <Stat size=StatSize::Lg trend=StatTrend::Increase>
                <StatLabel>"Large"</StatLabel>
                <StatBody>
                    <StatValue>"45K"</StatValue>
                    <StatDelta>"+23%"</StatDelta>
                </StatBody>
            </Stat>

            // Center aligned
            <Stat align=StatAlign::Center>
                <StatLabel>"Centered"</StatLabel>
                <StatValue>"999"</StatValue>
            </Stat>

            // Loading state
            <Stat loading=true>
                <StatLabel>"Loading..."</StatLabel>
                <StatValue>"12345"</StatValue>
            </Stat>

            // Complete example
            <Stat size=StatSize::Lg trend=StatTrend::Increase>
                <StatHeader>
                    <StatIcon>"📈"</StatIcon>
                    <StatLabel>"Total Growth"</StatLabel>
                </StatHeader>
                <StatBody>
                    <StatValue>"$89,432"</StatValue>
                    <StatDelta>"+18.2%"</StatDelta>
                </StatBody>
            </Stat>
        </div>
    }
}
