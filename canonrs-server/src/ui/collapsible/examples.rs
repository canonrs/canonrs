use leptos::prelude::*;
use super::collapsible_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Collapsible open=false>
            <CollapsibleTrigger>
                "Toggle Content"
            </CollapsibleTrigger>
            <CollapsibleContent>
                <p>"Collapsible content goes here."</p>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn BasicExample() -> impl IntoView {
    basic_example()
}
