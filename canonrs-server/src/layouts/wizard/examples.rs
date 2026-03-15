use leptos::prelude::*;
use super::WizardLayout;

pub fn basic_example() -> impl IntoView {
    view! {
        <WizardLayout
            header=leptos::children::ToChildren::to_children(|| view!{ <h2>"Wizard"</h2> })
            stepper=leptos::children::ToChildren::to_children(|| view!{ <div>"Step 1 · Step 2 · Step 3"</div> })
            footer=leptos::children::ToChildren::to_children(|| view!{ <div><button>"Next"</button></div> })
        >
            <p>"Step content"</p>
        </WizardLayout>
    }
}
