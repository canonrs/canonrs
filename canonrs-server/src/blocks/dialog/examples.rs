use leptos::prelude::*;
use super::dialog_block::DialogBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <DialogBlock
            header=leptos::children::ToChildren::to_children(|| view!{ <h3>"Dialog Title"</h3> })
            content=leptos::children::ToChildren::to_children(|| view!{ <p>"Dialog content"</p> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <button>"Close"</button> })
        />
    }
}
