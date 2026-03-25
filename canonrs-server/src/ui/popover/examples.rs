use leptos::prelude::*;
use super::popover_ui::*;
use crate::ui::button::button_ui::{Button, ButtonVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Popover>
            <Button variant=ButtonVariant::Primary attr:data-rs-popover-trigger="">"Open Popover"</Button>
            <PopoverContent>
                <p>"Popover content"</p>
            </PopoverContent>
        </Popover>
    }
}
