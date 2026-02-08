use leptos::prelude::*;
use super::link_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <p>
            "Check out our "
            <Link href="#".to_string()>"documentation"</Link>
            " for details."
        </p>
    }
}
