use leptos::prelude::*;
use super::collapsible_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Collapsible open=false id="collapsible-example-1".to_string()>
            <CollapsibleTrigger>
                "Toggle Content"
            </CollapsibleTrigger>
            <CollapsibleContent>
                <p>"Collapsible content goes here."</p>
            </CollapsibleContent>
        </Collapsible>
    }
}
