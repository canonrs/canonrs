use leptos::prelude::*;
use super::Columns;

pub fn basic_example() -> impl IntoView {
    view! {
        <Columns>
            <div data-block-region="col-1">"Column 1"</div>
            <div data-block-region="col-2">"Column 2"</div>
        </Columns>
    }
}
