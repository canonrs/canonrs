use leptos::prelude::*;
use super::callout_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Callout>
            <CalloutTitle>"Tip"</CalloutTitle>
            <CalloutDescription>"This is a helpful tip for users."</CalloutDescription>
        </Callout>
    }
}
