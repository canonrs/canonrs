use leptos::prelude::*;
use super::SidebarLayout;

pub fn basic_example() -> impl IntoView {
    view! {
        <SidebarLayout
            nav=leptos::children::ToChildren::to_children(|| view!{ <nav>"Navigation"</nav> })
            main=leptos::children::ToChildren::to_children(|| view!{ <main>"Main content"</main> })
        />
    }
}
