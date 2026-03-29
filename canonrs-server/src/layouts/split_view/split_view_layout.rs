//! @canon-id: split-view
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: left, right
//! @canon-label: Split View
//! @canon-icon: ◧
//! @canon-description: Left context panel and right action/detail panel
//! @canon-tags: split-view, split, columns, side-by-side, compare, dual
//! @canon-slot-accepts: left=Nav,right=Action
//! @canon-slot-descriptions: left:Context or list panel,right:Detail or action panel
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SplitRatio { #[default] Equal, FormFocused, ContextFocused }
impl SplitRatio {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equal          => "50-50",
            Self::FormFocused    => "40-60",
            Self::ContextFocused => "60-40",
        }
    }
}

#[component]
pub fn SplitViewLayout(
    #[prop(default = SplitRatio::Equal)] ratio: SplitRatio,
    #[prop(optional)] left: Option<AnyView>,
    #[prop(optional)] right: Option<AnyView>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-layout=""
            data-rs-component="SplitView"
            data-rs-variant=ratio.as_str()
            class=class
        >
            {left.map(|l| view! { <div data-rs-region="left">{l}</div> })}
            {right.map(|r| view! { <div data-rs-region="right">{r}</div> })}
        </div>
    }
}
