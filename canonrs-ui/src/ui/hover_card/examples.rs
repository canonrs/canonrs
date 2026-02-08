use leptos::prelude::*;
use super::hover_card_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <HoverCard open=Signal::from(false) id="hover-card-ex".to_string()>
                <HoverCardTrigger describedby_id="hover-card-ex".to_string() id="hover-card-trigger-ex".to_string()>
                    <button data-button data-ui-variant="link">"@username"</button>
                </HoverCardTrigger>
                <HoverCardContent open=Signal::from(false) content_id="hover-card-ex".to_string()>
                    <h4>"Username"</h4>
                    <p>"User bio and details."</p>
                </HoverCardContent>
            </HoverCard>
        </div>
    }
}
