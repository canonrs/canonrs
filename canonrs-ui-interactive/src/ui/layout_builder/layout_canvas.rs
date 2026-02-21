use leptos::prelude::*;
use canonrs_ui::layouts::{
    DashboardLayout, MarketingLayout, FullscreenLayout,
    SplitViewLayout, SplitRatio, WizardLayout, Section,
    PageLayout, PageLayoutVariant,
};
use leptos::children::ToChildren;
use super::types::{ActiveLayout, Node, DragContext, init_slots};
use super::drop_zone::DropZone;

#[component]
pub fn LayoutCanvas(
    layout: ActiveLayout,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    slots: RwSignal<Vec<Node>>,
) -> impl IntoView {
    let slot_id = move |name: &str| -> Option<uuid::Uuid> {
        let n = name.to_string();
        slots.get().iter().find(|s| s.label() == n).map(|s| s.id)
    };

    match layout {
        ActiveLayout::Dashboard => view! {
            <DashboardLayout>
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </DashboardLayout>
        }.into_any(),

        ActiveLayout::Marketing => view! {
            <MarketingLayout
                header_logo=ToChildren::to_children(|| view! { <div class="mock-region mock-region--header">"Logo"</div> })
                header_nav=ToChildren::to_children(|| view! { <div class="mock-region mock-region--header">"Nav"</div> })
                header_actions=ToChildren::to_children(|| view! { <div class="mock-region mock-region--header">"Actions"</div> })
                hero=ToChildren::to_children(move || view! {
                    {move || slot_id("hero").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="hero" /> })}
                })
                footer=ToChildren::to_children(move || view! {
                    {move || slot_id("footer").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="footer" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </MarketingLayout>
        }.into_any(),

        ActiveLayout::Fullscreen => view! {
            <FullscreenLayout
                header=ToChildren::to_children(move || view! {
                    {move || slot_id("header").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="header" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </FullscreenLayout>
        }.into_any(),

        ActiveLayout::SplitView => view! {
            <SplitViewLayout
                ratio=SplitRatio::Equal
                left=ToChildren::to_children(move || view! {
                    {move || slot_id("left").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="left" /> })}
                })
                right=ToChildren::to_children(move || view! {
                    {move || slot_id("right").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="right" /> })}
                })
            />
        }.into_any(),

        ActiveLayout::Wizard => view! {
            <WizardLayout
                header=ToChildren::to_children(move || view! {
                    {move || slot_id("header").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="header" /> })}
                })
                stepper=ToChildren::to_children(move || view! {
                    {move || slot_id("stepper").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="stepper" /> })}
                })
                footer=ToChildren::to_children(move || view! {
                    {move || slot_id("footer").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="footer" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </WizardLayout>
        }.into_any(),

        ActiveLayout::Section => view! {
            <Section
                header=ToChildren::to_children(move || view! {
                    {move || slot_id("header").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="header" /> })}
                })
                footer=ToChildren::to_children(move || view! {
                    {move || slot_id("footer").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="footer" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </Section>
        }.into_any(),

        ActiveLayout::PageSingle => view! {
            <PageLayout variant=PageLayoutVariant::Single>
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithSidebar => view! {
            <PageLayout
                variant=PageLayoutVariant::WithSidebar
                sidebar=ToChildren::to_children(move || view! {
                    {move || slot_id("sidebar").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="sidebar" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithAside => view! {
            <PageLayout
                variant=PageLayoutVariant::WithAside
                aside=ToChildren::to_children(move || view! {
                    {move || slot_id("aside").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="aside" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageSidebarAndAside => view! {
            <PageLayout
                variant=PageLayoutVariant::SidebarAndAside
                sidebar=ToChildren::to_children(move || view! {
                    {move || slot_id("sidebar").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="sidebar" /> })}
                })
                aside=ToChildren::to_children(move || view! {
                    {move || slot_id("aside").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="aside" /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id tree=tree drag_ctx=drag_ctx slot_label="main" /> })}
            </PageLayout>
        }.into_any(),
    }
}
