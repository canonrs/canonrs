//! Navigation Provider - Provides NavigationState via Leptos Context

use leptos::prelude::*;
use canonrs_shared::shared::navigation_context::NavigationState;

#[component]
pub fn NavigationProvider(
    children: Children,
) -> impl IntoView {
    let state = RwSignal::new(NavigationState::new());
    
    provide_context(state);
    
    view! {
        {children()}
    }
}

// Hook para consumir context
pub fn use_navigation_state() -> RwSignal<NavigationState> {
    use_context::<RwSignal<NavigationState>>()
        .expect("NavigationProvider not found in component tree")
}
