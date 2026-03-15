use leptos::prelude::*;
use super::{SplitViewLayout, SplitRatio};

pub fn basic_example() -> impl IntoView {
    view! {
        <SplitViewLayout
            ratio=SplitRatio::Equal
            left=leptos::children::ToChildren::to_children(|| view!{ <div>"Left panel"</div> })
            right=leptos::children::ToChildren::to_children(|| view!{ <div>"Right panel"</div> })
        />
    }
}
