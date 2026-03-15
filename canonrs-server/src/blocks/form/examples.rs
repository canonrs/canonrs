use leptos::prelude::*;
use super::FormBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <FormBlock>
            <div><label>"Email"</label><input type="email" placeholder="email@example.com" /></div>
            <div><label>"Password"</label><input type="password" /></div>
            <button type="submit">"Submit"</button>
        </FormBlock>
    }
}
