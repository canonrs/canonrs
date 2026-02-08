//! # SplitViewLayout
//! 
//! **Purpose:** Decision flows, authentication, onboarding, comparisons
//! **Coverage:** Login, Signup, Wizard, Side-by-side views
//! 
//! **Structure:**
//! ```
//! Left Panel (context/branding)
//! Right Panel (action/form)
//! ```
//! 
//! **Token Family:** Layout (layout.auth.*)
//! 
//! **Variants:**
//! - 50/50 split (default)
//! - 40/60 split (form-focused)
//! - 60/40 split (context-focused)

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SplitRatio {
    /// 50/50 split
    Equal,
    /// 40/60 split (form-focused)
    FormFocused,
    /// 60/40 split (context-focused)
    ContextFocused,
}

impl SplitRatio {
    fn as_str(&self) -> &'static str {
        match self {
            SplitRatio::Equal => "50-50",
            SplitRatio::FormFocused => "40-60",
            SplitRatio::ContextFocused => "60-40",
        }
    }
}

#[component]
pub fn SplitViewLayout(
    /// Split ratio
    #[prop(default = SplitRatio::Equal)]
    ratio: SplitRatio,
    /// Left panel content (context/branding)
    left: Children,
    /// Right panel content (action/form)
    right: Children,
) -> impl IntoView {
    view! {
        <div
            class="layout-split-view"
            attr:data-layout="split-view"
            attr:data-split-ratio=ratio.as_str()
        >
            <div
                class="layout-split-left"
                attr:data-layout-region="left"
            >
                {left()}
            </div>

            <div
                class="layout-split-right"
                attr:data-layout-region="right"
            >
                {right()}
            </div>
        </div>
    }
}
