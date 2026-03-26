use leptos::prelude::*;
use super::FullscreenLayout;

pub fn basic_example() -> impl IntoView {
    view! {
        <FullscreenLayout
            header=leptos::children::ToChildren::to_children(|| view!{ <nav>"Header"</nav> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Fullscreen content"</p> })
        />
    }
}
