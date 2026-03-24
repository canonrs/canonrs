use leptos::prelude::*;
use super::DrawerBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <DrawerBlock open=Signal::derive(|| true)
            header=leptos::children::ToChildren::to_children(|| view!{ <span>"Drawer Title"</span> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <button>"Close"</button> })
        >
            <p>"Drawer content."</p>
        </DrawerBlock>
    }
}
