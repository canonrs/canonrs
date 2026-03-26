use leptos::prelude::*;
use super::stat_card_block::{StatCard, StatTrend};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:1rem;">
            <StatCard trend=StatTrend::Up
                label=leptos::children::ToChildren::to_children(|| view!{ "Total Revenue" })
                value=leptos::children::ToChildren::to_children(|| view!{ "$48,295" })
                change=leptos::children::ToChildren::to_children(|| view!{ "+12.5%" })
            />
            <StatCard trend=StatTrend::Down
                label=leptos::children::ToChildren::to_children(|| view!{ "Bounce Rate" })
                value=leptos::children::ToChildren::to_children(|| view!{ "3.2%" })
                change=leptos::children::ToChildren::to_children(|| view!{ "-0.5%" })
            />
            <StatCard
                label=leptos::children::ToChildren::to_children(|| view!{ "Active Users" })
                value=leptos::children::ToChildren::to_children(|| view!{ "2,350" })
            />
        </div>
    }
}
