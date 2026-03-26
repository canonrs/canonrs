use leptos::prelude::*;
use super::command_panel_block::CommandPanelBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <CommandPanelBlock
            search=leptos::children::ToChildren::to_children(|| view!{ <input type="text" placeholder="Search..." /> })
            results=leptos::children::ToChildren::to_children(|| view!{
                <div>"Result 1"</div>
                <div>"Result 2"</div>
            })
        />
    }
}
