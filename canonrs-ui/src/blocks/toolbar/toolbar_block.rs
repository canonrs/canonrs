//! # Toolbar Block
//! Container for actions and tools

use leptos::prelude::*;

#[component]
pub fn ToolbarBlock(
    #[prop(optional)] left: Option<Children>,
    #[prop(optional)] center: Option<Children>,
    #[prop(optional)] right: Option<Children>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div 
            class=format!("canon-toolbar {}", class)
            attr:data-block="toolbar"
            role="toolbar"
        >
            {left.map(|l| view! {
                <div class="canon-toolbar__section canon-toolbar__section--left">
                    {l()}
                </div>
            })}
            
            {center.map(|c| view! {
                <div class="canon-toolbar__section canon-toolbar__section--center">
                    {c()}
                </div>
            })}
            
            <div class="canon-toolbar__section canon-toolbar__section--main">
                {children()}
            </div>
            
            {right.map(|r| view! {
                <div class="canon-toolbar__section canon-toolbar__section--right">
                    {r()}
                </div>
            })}
        </div>
    }
}
