use leptos::prelude::*;
use super::{PageLayout, PageLayoutVariant};

pub fn basic_example() -> impl IntoView {
    view! {
        <PageLayout variant=PageLayoutVariant::Single>
            <p>"Page content"</p>
        </PageLayout>
    }
}

pub fn with_sidebar_example() -> impl IntoView {
    view! {
        <PageLayout
            variant=PageLayoutVariant::WithSidebar
            sidebar=leptos::children::ToChildren::to_children(|| view!{ <nav>"Sidebar"</nav> })
        >
            <p>"Main content"</p>
        </PageLayout>
    }
}
