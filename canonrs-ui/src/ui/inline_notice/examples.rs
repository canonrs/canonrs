use leptos::prelude::*;
use super::inline_notice_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <InlineNotice>
            "This is an inline notice message."
        </InlineNotice>
    }
}
