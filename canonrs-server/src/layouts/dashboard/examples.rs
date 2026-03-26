use leptos::prelude::*;
use super::DashboardLayout;

pub fn basic_example() -> impl IntoView {
    view! {
        <DashboardLayout
            header=leptos::children::ToChildren::to_children(|| view!{ <nav>"Header nav"</nav> })
            sidebar=leptos::children::ToChildren::to_children(|| view!{ <nav>"Sidebar nav"</nav> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Main content"</p> })
        />
    }
}
