use leptos::prelude::*;
use canonrs_ui::layouts::{
    DashboardLayout, MarketingLayout, FullscreenLayout,
    SplitViewLayout, SplitRatio, WizardLayout, Section,
    PageLayout, PageLayoutVariant,
};
use leptos::children::ToChildren;
use super::types::{ActiveLayout, DroppedBlock, LayoutRegion, DragContext};
use super::drop_zone::DropZone;

#[component]
pub fn LayoutCanvas(
    layout: ActiveLayout,
    dropped: RwSignal<Vec<DroppedBlock>>,
    drag_ctx: RwSignal<DragContext>,
) -> impl IntoView {
    match layout {
        ActiveLayout::Dashboard => view! {
            <DashboardLayout>
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </DashboardLayout>
        }.into_any(),

        ActiveLayout::Marketing => view! {
            <MarketingLayout
                header_logo=ToChildren::to_children(|| view! {
                    <div class="mock-region mock-region--header">"Logo"</div>
                })
                header_nav=ToChildren::to_children(|| view! {
                    <div class="mock-region mock-region--header">"Nav"</div>
                })
                header_actions=ToChildren::to_children(|| view! {
                    <div class="mock-region mock-region--header">"Actions"</div>
                })
                hero=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Hero dropped=dropped drag_ctx=drag_ctx />
                })
                footer=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Footer dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </MarketingLayout>
        }.into_any(),

        ActiveLayout::Fullscreen => view! {
            <FullscreenLayout
                header=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Header dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </FullscreenLayout>
        }.into_any(),

        ActiveLayout::SplitView => view! {
            <SplitViewLayout
                ratio=SplitRatio::Equal
                left=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Left dropped=dropped drag_ctx=drag_ctx />
                })
                right=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Right dropped=dropped drag_ctx=drag_ctx />
                })
            />
        }.into_any(),

        ActiveLayout::Wizard => view! {
            <WizardLayout
                header=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Header dropped=dropped drag_ctx=drag_ctx />
                })
                stepper=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Stepper dropped=dropped drag_ctx=drag_ctx />
                })
                footer=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Footer dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </WizardLayout>
        }.into_any(),

        ActiveLayout::Section => view! {
            <Section
                header=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Header dropped=dropped drag_ctx=drag_ctx />
                })
                footer=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Footer dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </Section>
        }.into_any(),

        ActiveLayout::PageSingle => view! {
            <PageLayout variant=PageLayoutVariant::Single>
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithSidebar => view! {
            <PageLayout
                variant=PageLayoutVariant::WithSidebar
                sidebar=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Sidebar dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithAside => view! {
            <PageLayout
                variant=PageLayoutVariant::WithAside
                aside=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Aside dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageSidebarAndAside => view! {
            <PageLayout
                variant=PageLayoutVariant::SidebarAndAside
                sidebar=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Sidebar dropped=dropped drag_ctx=drag_ctx />
                })
                aside=ToChildren::to_children(move || view! {
                    <DropZone region=LayoutRegion::Aside dropped=dropped drag_ctx=drag_ctx />
                })
            >
                <DropZone region=LayoutRegion::Main dropped=dropped drag_ctx=drag_ctx />
            </PageLayout>
        }.into_any(),
    }
}
