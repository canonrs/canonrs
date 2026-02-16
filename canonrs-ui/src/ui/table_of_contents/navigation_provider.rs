//! Navigation Context Provider for Leptos
//! Provides global navigation state to all doc components

use leptos::prelude::*;
use canonrs_shared::{NavigationState, HeadingHierarchy, TocItem};

// ── Context ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct NavigationContext {
    pub state: RwSignal<NavigationState>,
}

impl NavigationContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(NavigationState::new()),
        }
    }

    pub fn update_heading(&self, heading_id: String) {
        self.state.update(|state| {
            state.update_active_heading(heading_id);
        });
    }

    pub fn update_progress(&self, progress: f32) {
        self.state.update(|state| {
            state.update_scroll_progress(progress);
        });
    }

    pub fn get_breadcrumb(&self) -> Vec<(String, String)> {
        self.state.with(|state| {
            state.current_heading_id
                .as_ref()
                .map(|id| state.heading_hierarchy.get_breadcrumb(id))
                .unwrap_or_default()
        })
    }

    pub fn get_progress(&self) -> f32 {
        self.state.with(|state| state.scroll_progress)
    }
}

// ── Provider Component ───────────────────────────────────────────────────────

#[component]
pub fn NavigationProvider(
    toc_items: Vec<TocItem>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = NavigationContext::new();
    
    // Initialize hierarchy from TOC items
    context.state.update(|state| {
        state.heading_hierarchy = HeadingHierarchy::from_toc_items(&toc_items);
    });

    provide_context(context);

    view! {
        {children.map(|c| c())}
    }
}

// ── Hook to use context ──────────────────────────────────────────────────────

pub fn use_navigation() -> NavigationContext {
    expect_context::<NavigationContext>()
}
