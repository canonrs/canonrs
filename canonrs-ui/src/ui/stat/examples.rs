use leptos::prelude::*;
use super::stat_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 2rem;">
            <Stat>
                <StatLabel>"Revenue"</StatLabel>
                <StatValue>"$45,231"</StatValue>
            </Stat>
            <Stat>
                <StatLabel>"Users"</StatLabel>
                <StatValue>"2,350"</StatValue>
            </Stat>
        </div>
    }
}
