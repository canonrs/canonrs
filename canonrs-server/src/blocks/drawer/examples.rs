use leptos::prelude::*;
use super::drawer_block::DrawerBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <DrawerBlock
            header=leptos::children::ToChildren::to_children(|| view!{ <h3>"Drawer Title"</h3> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Drawer content"</p> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <button>"Close"</button> })
        />
    }
}
