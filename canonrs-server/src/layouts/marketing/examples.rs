use leptos::prelude::*;
use super::MarketingLayout;

pub fn basic_example() -> impl IntoView {
    view! {
        <MarketingLayout
            header=leptos::children::ToChildren::to_children(|| view!{ <nav>"Header"</nav> })
            hero=leptos::children::ToChildren::to_children(|| view!{ <div>"Hero section"</div> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <footer>"Footer"</footer> })
        >
            <p>"Main content"</p>
        </MarketingLayout>
    }
}
