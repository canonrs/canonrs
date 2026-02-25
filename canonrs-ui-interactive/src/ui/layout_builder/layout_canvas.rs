use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use canonrs_ui::layouts::{
    DashboardLayout, MarketingLayout, FullscreenLayout,
    SplitViewLayout, SplitRatio, WizardLayout, Section,
    PageLayout, PageLayoutVariant,
};
use leptos::children::ToChildren;
use super::types::{ActiveLayout, Node, DragContext, CanvasMode, init_slots};
use super::ui::drop_zone::DropZone;

#[component]
pub fn LayoutCanvas(
    layout: ActiveLayout,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    slots: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<super::ui::drop_zone::DragVisualState>,
) -> impl IntoView {
    let slot_id = move |name: &str| -> Option<uuid::Uuid> {
        let n = name.to_string();
        slots.get().iter().find(|s| s.label() == n).map(|s| s.id)
    };

    match layout {
        ActiveLayout::Dashboard => view! {
            <DashboardLayout>
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </DashboardLayout>
        }.into_any(),

        ActiveLayout::Marketing => view! {
            <MarketingLayout
                header_logo=ToChildren::to_children(|| view! { <div class="mock-region mock-region--header">"Logo"</div> })
                header_nav=ToChildren::to_children(|| view! { <div class="mock-region mock-region--header">"Nav"</div> })
                header_actions=ToChildren::to_children(|| view! { <div class="mock-region mock-region--header">"Actions"</div> })
                hero=ToChildren::to_children(move || view! {
                    {move || slot_id("hero").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="hero".to_string() /> })}
                })
                footer=ToChildren::to_children(move || view! {
                    {move || slot_id("footer").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="footer".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </MarketingLayout>
        }.into_any(),

        ActiveLayout::Fullscreen => view! {
            <FullscreenLayout
                header=ToChildren::to_children(move || view! {
                    {move || slot_id("header").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="header".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </FullscreenLayout>
        }.into_any(),

        ActiveLayout::SplitView => view! {
            <SplitViewLayout
                ratio=SplitRatio::Equal
                left=ToChildren::to_children(move || view! {
                    {move || slot_id("left").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="left".to_string() /> })}
                })
                right=ToChildren::to_children(move || view! {
                    {move || slot_id("right").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="right".to_string() /> })}
                })
            />
        }.into_any(),

        ActiveLayout::Wizard => view! {
            <WizardLayout
                header=ToChildren::to_children(move || view! {
                    {move || slot_id("header").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="header".to_string() /> })}
                })
                stepper=ToChildren::to_children(move || view! {
                    {move || slot_id("stepper").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="stepper".to_string() /> })}
                })
                footer=ToChildren::to_children(move || view! {
                    {move || slot_id("footer").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="footer".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </WizardLayout>
        }.into_any(),

        ActiveLayout::Section => view! {
            <Section
                header=ToChildren::to_children(move || view! {
                    {move || slot_id("header").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="header".to_string() /> })}
                })
                footer=ToChildren::to_children(move || view! {
                    {move || slot_id("footer").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="footer".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </Section>
        }.into_any(),

        ActiveLayout::PageSingle => view! {
            <PageLayout variant=PageLayoutVariant::Single>
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithSidebar => view! {
            <PageLayout
                variant=PageLayoutVariant::WithSidebar
                sidebar=ToChildren::to_children(move || view! {
                    {move || slot_id("sidebar").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="sidebar".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithAside => view! {
            <PageLayout
                variant=PageLayoutVariant::WithAside
                aside=ToChildren::to_children(move || view! {
                    {move || slot_id("aside").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="aside".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageSidebarAndAside => view! {
            <PageLayout
                variant=PageLayoutVariant::SidebarAndAside
                sidebar=ToChildren::to_children(move || view! {
                    {move || slot_id("sidebar").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="sidebar".to_string() /> })}
                })
                aside=ToChildren::to_children(move || view! {
                    {move || slot_id("aside").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="aside".to_string() /> })}
                })
            >
                {move || slot_id("main").map(|id| view! { <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual slot_label="main".to_string() virtualize=true /> })}
            </PageLayout>
        }.into_any(),
    }
}
