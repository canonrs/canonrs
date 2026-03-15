use leptos::prelude::*;
use super::Grid;

pub fn basic_example() -> impl IntoView {
    view! {
        <Grid columns=3>
            <div>"Cell 1"</div>
            <div>"Cell 2"</div>
            <div>"Cell 3"</div>
        </Grid>
    }
}
