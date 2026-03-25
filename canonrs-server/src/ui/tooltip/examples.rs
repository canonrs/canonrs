use leptos::prelude::*;
use super::tooltip_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger>"Hover me"</TooltipTrigger>
                <TooltipContent>"This is a tooltip"</TooltipContent>
            </Tooltip>
        </TooltipProvider>
    }
}
