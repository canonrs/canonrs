use leptos::prelude::*;
use super::{PageLayout, PageLayoutVariant};

pub fn basic_example() -> impl IntoView {
    view! {
        <PageLayout
            variant=PageLayoutVariant::Single
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Page content"</p> })
        />
    }
}

pub fn with_sidebar_example() -> impl IntoView {
    view! {
        <PageLayout
            variant=PageLayoutVariant::WithSidebar
            sidebar=leptos::children::ToChildren::to_children(|| view!{ <nav>"Sidebar"</nav> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Main content"</p> })
        />
    }
}
