use leptos::prelude::*;
use super::popover_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <Popover open=Signal::from(false) id="popover-ex".to_string()>
                <PopoverTrigger controls_id="popover-ex".to_string() id="popover-trigger-ex".to_string()>
                    <button data-button data-ui-variant="default">"Open Popover"</button>
                </PopoverTrigger>
                <PopoverContent content_id="popover-ex".to_string()>
                    <p>"Popover content goes here."</p>
                </PopoverContent>
            </Popover>
        </div>
    }
}
