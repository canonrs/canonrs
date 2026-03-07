use leptos::prelude::*;
use crate::providers::layout_types::{LayoutContext, LayoutMode};

#[component]
pub fn LayoutProvider(children: Children) -> impl IntoView {
    let mode = RwSignal::new(LayoutMode::default());

    provide_context(LayoutContext { mode });

    view! { {children()} }
}
