use leptos::prelude::*;
use super::Footer;

pub fn basic_example() -> impl IntoView {
    view! {
        <Footer
            left=leptos::children::ToChildren::to_children(|| view!{ <span>"© 2024 CanonRS"</span> })
            center=leptos::children::ToChildren::to_children(|| view!{ <span>"Terms · Privacy"</span> })
            right=leptos::children::ToChildren::to_children(|| view!{ <span>"v1.0.0"</span> })
        />
    }
}
