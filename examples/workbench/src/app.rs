use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::providers::CanonShell;
use crate::pages::{
    showcase::index::ShowcasePage,
    theme::index::ThemePage,
    playground::index::PlaygroundPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="canonrs" href="/pkg/canonrs-ui.css"/>
        <Title text="CanonRS UI - Design System"/>
        <Meta name="description" content="Modern UI Framework built with Leptos"/>

        <Router>
            <CanonShell>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=ShowcasePage/>
                    <Route path=path!("/theme") view=ThemePage/>
                    <Route path=path!("/playground") view=PlaygroundPage/>
                </Routes>
            </CanonShell>
        </Router>
    }
}
