use leptos::prelude::*;
use super::Toolbar;
use crate::ui::button::Button;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Toolbar>
            <Button>"Cut"</Button>
            <Button>"Copy"</Button>
            <Button>"Paste"</Button>
        </Toolbar>
    }
}
