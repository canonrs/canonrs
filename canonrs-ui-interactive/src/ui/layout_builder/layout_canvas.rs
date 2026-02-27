use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use canonrs_ui::layouts::{
    DashboardLayout, MarketingLayout, FullscreenLayout,
    SplitViewLayout, SplitRatio, WizardLayout, Section,
    PageLayout, PageLayoutVariant,
};
use super::types::{ActiveLayout, Node, DragContext, CanvasMode};
use super::ui::drop_zone::{DropZone, DragVisualState};

fn region_zone(
    slot_name: &'static str,
    slots: RwSignal<Vec<Node>>,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
) -> impl IntoView {
    let parent_id = move || {
        slots.get().iter().find(|s| s.label() == slot_name).map(|s| s.id)
    };

    view! {
        {move || parent_id().map(|pid| view! {
            <DropZone
                parent_id=pid
                engine=engine
                tree=tree
                drag_ctx=drag_ctx
                selected_id=selected_id
                canvas_mode=canvas_mode
                drag_visual=drag_visual
                slot_label=slot_name.to_string()
            />
        })}
    }
}

#[component]
pub fn LayoutCanvas(
    layout: ActiveLayout,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    slots: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
) -> impl IntoView {
    macro_rules! rz {
        ($slot:expr) => {
            region_zone($slot, slots, engine, tree, drag_ctx, selected_id, canvas_mode, drag_visual)
        };
    }
    macro_rules! rz_children {
        ($slot:expr) => {
            leptos::children::ToChildren::to_children(move || rz!($slot))
        };
    }

    match layout {
        ActiveLayout::Dashboard => view! {
            <DashboardLayout
                header=rz_children!("header")
                sidebar=rz_children!("sidebar")
            >
                {rz!("main")}
            </DashboardLayout>
        }.into_any(),

        ActiveLayout::Marketing => view! {
            <MarketingLayout
                header=rz_children!("header")
                hero=rz_children!("hero")
                footer=rz_children!("footer")
            >
                {rz!("main")}
            </MarketingLayout>
        }.into_any(),

        ActiveLayout::Fullscreen => view! {
            <FullscreenLayout header=rz_children!("header")>
                {rz!("main")}
            </FullscreenLayout>
        }.into_any(),

        ActiveLayout::SplitView => view! {
            <SplitViewLayout
                ratio=SplitRatio::Equal
                left=leptos::children::ToChildren::to_children(move || rz!("left"))
                right=leptos::children::ToChildren::to_children(move || rz!("right"))
            />
        }.into_any(),

        ActiveLayout::Wizard => view! {
            <WizardLayout
                header=rz_children!("header")
                stepper=rz_children!("stepper")
                footer=rz_children!("footer")
            >
                {rz!("main")}
            </WizardLayout>
        }.into_any(),

        ActiveLayout::Section => view! {
            <Section
                header=rz_children!("header")
                footer=rz_children!("footer")
            >
                {rz!("main")}
            </Section>
        }.into_any(),

        ActiveLayout::PageSingle => view! {
            <PageLayout variant=PageLayoutVariant::Single>
                {rz!("main")}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithSidebar => view! {
            <PageLayout
                variant=PageLayoutVariant::WithSidebar
                sidebar=rz_children!("sidebar")
            >
                {rz!("main")}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithAside => view! {
            <PageLayout
                variant=PageLayoutVariant::WithAside
                aside=rz_children!("aside")
            >
                {rz!("main")}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageSidebarAndAside => view! {
            <PageLayout
                variant=PageLayoutVariant::SidebarAndAside
                sidebar=rz_children!("sidebar")
                aside=rz_children!("aside")
            >
                {rz!("main")}
            </PageLayout>
        }.into_any(),
    }
}
