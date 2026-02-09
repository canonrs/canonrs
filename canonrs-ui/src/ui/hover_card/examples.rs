use leptos::prelude::*;
use super::hover_card_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <HoverCard id="hover-card-ex".to_string()>
                <HoverCardTrigger target_hover_card_id="hover-card-ex".to_string()>
                    <span class="text-blue-600 underline cursor-pointer">"@username"</span>
                </HoverCardTrigger>
                <HoverCardContent>
                    <div>
                        <h4 class="font-semibold">"User Profile"</h4>
                        <p class="text-sm mt-1">"Software developer."</p>
                    </div>
                </HoverCardContent>
            </HoverCard>
        </div>
    }
}
