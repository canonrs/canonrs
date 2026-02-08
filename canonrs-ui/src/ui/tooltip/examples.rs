use leptos::prelude::*;
use super::tooltip_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div>
            <TooltipProvider>
                <Tooltip open=Signal::from(false) id="tooltip-ex".to_string()>
                    <TooltipTrigger describedby_id="tooltip-ex".to_string() id="tooltip-trigger-ex".to_string()>
                        <button data-button data-ui-variant="default">"Hover me"</button>
                    </TooltipTrigger>
                    <TooltipContent content_id="tooltip-ex".to_string()>
                        "This is a tooltip"
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
        </div>
    }
}
