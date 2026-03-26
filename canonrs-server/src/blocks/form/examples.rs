use leptos::prelude::*;
use super::form_block::FormBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <FormBlock
            fields=leptos::children::ToChildren::to_children(|| view!{
                <div><label>"Email"</label><input type="email" placeholder="email@example.com" /></div>
                <div><label>"Password"</label><input type="password" /></div>
            })
            actions=leptos::children::ToChildren::to_children(|| view!{
                <button type="submit">"Submit"</button>
            })
        />
    }
}
