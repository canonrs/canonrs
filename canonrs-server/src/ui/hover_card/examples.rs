use leptos::prelude::*;
use super::hover_card_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger>"Hover me"</HoverCardTrigger>
            <HoverCardContent>
                <p>"HoverCard content"</p>
            </HoverCardContent>
        </HoverCard>
    }
}
