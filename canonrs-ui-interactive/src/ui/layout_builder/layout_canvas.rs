use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use canonrs_ui::layouts::{
    DashboardLayout, MarketingLayout, FullscreenLayout,
    SplitViewLayout, SplitRatio, WizardLayout, Section,
    PageLayout, PageLayoutVariant,
};
use leptos::children::ToChildren;
use super::types::{ActiveLayout, Node, NodeKind, DragContext, CanvasMode};
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

    let layout_node_id = move || {
        tree.get().iter().find(|n| matches!(&n.kind, NodeKind::Layout { .. })).map(|n| n.id)
    };

    macro_rules! dz {
        ($slot:expr) => {
            move || slot_id($slot).map(|id| view! {
                <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx
                    selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual
                    slot_label=$slot.to_string() />
            })
        };
        ($slot:expr, virt) => {
            move || slot_id($slot).map(|id| view! {
                <DropZone parent_id=id engine=engine tree=tree drag_ctx=drag_ctx
                    selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual
                    slot_label=$slot.to_string() virtualize=true />
            })
        };
    }

    match layout {
        ActiveLayout::Dashboard => view! {
            <DashboardLayout
                header=ToChildren::to_children(dz!("header"))
                sidebar=ToChildren::to_children(dz!("sidebar"))
            >
                {dz!("main", virt)()}
            </DashboardLayout>
        }.into_any(),

        ActiveLayout::Marketing => view! {
            <MarketingLayout
                header=ToChildren::to_children(dz!("header"))
                hero=ToChildren::to_children(dz!("hero"))
                footer=ToChildren::to_children(dz!("footer"))
            >
                {dz!("main", virt)()}
            </MarketingLayout>
        }.into_any(),

        ActiveLayout::Fullscreen => view! {
            <FullscreenLayout
                header=ToChildren::to_children(dz!("header"))
            >
                {dz!("main", virt)()}
            </FullscreenLayout>
        }.into_any(),

        ActiveLayout::SplitView => view! {
            <SplitViewLayout
                ratio=SplitRatio::Equal
                left=ToChildren::to_children(dz!("left"))
                right=ToChildren::to_children(dz!("right"))
            />
        }.into_any(),

        ActiveLayout::Wizard => view! {
            <WizardLayout
                header=ToChildren::to_children(dz!("header"))
                stepper=ToChildren::to_children(dz!("stepper"))
                footer=ToChildren::to_children(dz!("footer"))
            >
                {dz!("main", virt)()}
            </WizardLayout>
        }.into_any(),

        ActiveLayout::Section => view! {
            <Section
                header=ToChildren::to_children(dz!("header"))
                footer=ToChildren::to_children(dz!("footer"))
            >
                {dz!("main", virt)()}
            </Section>
        }.into_any(),

        ActiveLayout::PageSingle => view! {
            <PageLayout variant=PageLayoutVariant::Single>
                {dz!("main", virt)()}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithSidebar => view! {
            <PageLayout
                variant=PageLayoutVariant::WithSidebar
                sidebar=ToChildren::to_children(dz!("sidebar"))
            >
                {dz!("main", virt)()}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithAside => view! {
            <PageLayout
                variant=PageLayoutVariant::WithAside
                aside=ToChildren::to_children(dz!("aside"))
            >
                {dz!("main", virt)()}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageSidebarAndAside => view! {
            <PageLayout
                variant=PageLayoutVariant::SidebarAndAside
                sidebar=ToChildren::to_children(dz!("sidebar"))
                aside=ToChildren::to_children(dz!("aside"))
            >
                {dz!("main", virt)()}
            </PageLayout>
        }.into_any(),
    }
}
