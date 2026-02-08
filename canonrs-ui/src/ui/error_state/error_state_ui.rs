//! ErrorState UI Component
//! Tratamento padronizado de estados de erro

use leptos::prelude::*;
use crate::primitives::error_state::ErrorStatePrimitive;

#[component]
pub fn ErrorState(
    #[prop(default = "Something went wrong".to_string())] title: String,
    #[prop(default = "An error occurred. Please try again.".to_string())] message: String,
    #[prop(optional)] icon: Option<Children>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] retry_button: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ErrorStatePrimitive class={class} id={id}>
            {icon.map(|i| view! {
                <div attr:data-error-state-icon="">
                    {i()}
                </div>
            })}
            
            <h3 attr:data-error-state-title="">{title}</h3>
            <p attr:data-error-state-description="">{message}</p>
            
            {children.map(|c| c())}
            
            {retry_button.map(|btn| view! {
                <div attr:data-error-state-actions="">
                    {btn()}
                </div>
            })}
        </ErrorStatePrimitive>
    }
}
