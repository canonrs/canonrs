//! Tabs Component
//!
//! @canon-level: loose
//! @canon-exceptions: [#24]
//! @canon-justification: Precise padding for tab alignment
//! @canon-owner: ui-team
//! @canon-target-date: 2025-02-28
//! @canon-migration-status: planned

use crate::primitives::{
    TabsContentPrimitive, TabsListPrimitive, TabsPrimitive, TabsTriggerPrimitive,
};
use crate::tokens::SEMANTIC;
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Tabs(
    children: Children,
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!("flex flex-col gap-2", class);

    view! {
        <TabsPrimitive value=value>
            <div class=merged_class>
                {children()}
            </div>
        </TabsPrimitive>
    }
}

#[component]
pub fn TabsList(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        format!("bg-[{}] text-[{}] inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px]",
            SEMANTIC.muted, SEMANTIC.muted_foreground),
        class
    );

    view! {
        <TabsListPrimitive>
            <div class=merged_class>
                {children()}
            </div>
        </TabsListPrimitive>
    }
}

#[component]
pub fn TabsTrigger(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        format!(
            "data-[state=active]:bg-[{}] data-[state=active]:text-[{}] \
             inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 \
             rounded-md border border-transparent px-2 py-1 text-sm font-medium \
             whitespace-nowrap transition-colors data-[state=active]:shadow-sm \
             cursor-pointer hover:bg-accent hover:text-accent-foreground",
            SEMANTIC.background, SEMANTIC.foreground
        ),
        class
    );

    view! {
        <TabsTriggerPrimitive value=value class=merged_class>
            {children()}
        </TabsTriggerPrimitive>
    }
}

#[component]
pub fn TabsContent(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!("flex-1 outline-none", class);

    view! {
        <TabsContentPrimitive value=value>
            <div class=merged_class>
                {children()}
            </div>
        </TabsContentPrimitive>
    }
}
