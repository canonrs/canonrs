use leptos::prelude::*;
use super::Wizard;

pub fn basic_example() -> impl IntoView {
    view! {
        <Wizard
            steps=leptos::children::ToChildren::to_children(|| view!{
                <div>"Step 1 · Step 2 · Step 3"</div>
            })
            body=leptos::children::ToChildren::to_children(|| view!{
                <div>"Step content here."</div>
            })
            actions=leptos::children::ToChildren::to_children(|| view!{
                <button>"Next"</button>
            })
        />
    }
}
