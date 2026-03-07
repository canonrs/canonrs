use leptos::prelude::*;
use super::tooltip_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div>
            <TooltipProvider>
                <Tooltip id="tooltip-ex".to_string()>
                    <TooltipTrigger target_tooltip_id="tooltip-ex".to_string()>
                        <button data-button data-ui-variant="solid">"Hover me"</button>
                    </TooltipTrigger>
                    <TooltipContent>
                        "This is a tooltip"
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
        </div>
    }
}
