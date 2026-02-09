use leptos::prelude::*;
use super::popover_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <Popover id="popover-ex".to_string()>
                <PopoverTrigger target_popover_id="popover-ex".to_string()>
                    <button data-button data-ui-variant="solid">"Open Popover"</button>
                </PopoverTrigger>
                <PopoverContent>
                    <div class="p-4">
                        <h3 class="font-semibold mb-2">"Popover Title"</h3>
                        <p class="text-sm">"This is popover content."</p>
                    </div>
                </PopoverContent>
            </Popover>
        </div>
    }
}
