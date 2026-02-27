use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use canonrs_ui::layouts::{
    DashboardLayout, MarketingLayout, FullscreenLayout,
    SplitViewLayout, SplitRatio, WizardLayout, Section,
    PageLayout, PageLayoutVariant,
};
use super::types::{ActiveLayout, Node, NodeKind, DragContext, CanvasMode, children_of};
use super::ui::drop_zone::{DropZone, DragVisualState};
use super::ui::block_preview::BlockPreview;

/// Renderiza o conteúdo de uma região diretamente — sem wrapper.
/// A região (header, aside, main...) já tem data-drop-zone no layout.
#[derive(Clone, Copy, PartialEq)]
enum RegionOrientation { Row, Column }

fn region_children(
    slot_name: &'static str,
    orientation: RegionOrientation,
    slots: RwSignal<Vec<Node>>,
    tree: RwSignal<Vec<Node>>,
    engine: RwSignal<BuilderEngine>,
    drag_ctx: RwSignal<DragContext>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
) -> impl IntoView {
    let slot_id = move || {
        slots.get().iter().find(|s| s.label() == slot_name).map(|s| s.id)
    };
    let is_builder = move || canvas_mode.get() == CanvasMode::Builder;
    let is_active = move || drag_visual.get().active_zone_id == slot_id();
    let insert_idx = move || drag_visual.get().insert_index;
    let is_dragging = move || drag_ctx.get().is_dragging() && drag_ctx.get().layout_def.is_none();

    let insert_line = move |pos: usize| view! {
        <div style=move || if is_active() && is_builder() && insert_idx() == pos {
            if orientation == RegionOrientation::Row {
                "width:4px;height:100%;min-height:24px;background:var(--builder-insert-line-color);border-radius:2px;margin:0 2px;pointer-events:none;align-self:stretch;"
            } else {
                "height:4px;background:var(--builder-insert-line-color);border-radius:2px;margin:2px 0;pointer-events:none;"
            }
        } else {
            if orientation == RegionOrientation::Row {
                "width:2px;background:transparent;margin:0 1px;pointer-events:none;"
            } else {
                "height:2px;background:transparent;margin:1px 0;pointer-events:none;"
            }
        } />
    };

    let children_memo = Memo::new(move |_| {
        slot_id().map(|id| children_of(&tree.get(), id)).unwrap_or_default()
    });

    view! {
        {move || {
            let nodes = children_memo.get();
            let empty = nodes.is_empty();
            view! {
                {insert_line(0)}
                <For
                    each=move || children_memo.get()
                    key=|n| n.id
                    children=move |n| {
                        let pos = children_memo.get().iter().position(|c| c.id == n.id).unwrap_or(0);
                        view! {
                            <BlockPreview node=n engine=engine tree=tree drag_ctx=drag_ctx
                                selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual />
                            {insert_line(pos + 1)}
                        }
                    }
                />
                {move || if children_memo.get().is_empty() && is_builder() {
                    Some(view! {
                        <div data-drop-zone-empty="">
                            {slot_name}
                        </div>
                    })
                } else { None }}
            }
        }}
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
    let slot_id = move |name: &str| -> Option<String> {
        let n = name.to_string();
        slots.get().iter().find(|s| s.label() == n).map(|s| s.id.to_string())
    };
    let zid = move |name: &'static str| -> Signal<String> {
        Signal::derive(move || slot_id(name).unwrap_or_default())
    };

    macro_rules! rc {
        ($slot:expr) => {
            region_children($slot, RegionOrientation::Column, slots, tree, engine, drag_ctx, selected_id, canvas_mode, drag_visual)
        };
        ($slot:expr, row) => {
            region_children($slot, RegionOrientation::Row, slots, tree, engine, drag_ctx, selected_id, canvas_mode, drag_visual)
        };
    }



    match layout {
        ActiveLayout::Dashboard => view! {
            <DashboardLayout
                header_zone_id=zid("header")
                sidebar_zone_id=zid("sidebar")
                main_zone_id=zid("main")
                header=leptos::children::ToChildren::to_children(move || rc!("header", row))
                sidebar=leptos::children::ToChildren::to_children(move || rc!("sidebar"))
            >
                {rc!("main")}
            </DashboardLayout>
        }.into_any(),

        ActiveLayout::Marketing => view! {
            <MarketingLayout
                header_zone_id=zid("header")
                hero_zone_id=zid("hero")
                main_zone_id=zid("main")
                footer_zone_id=zid("footer")
                header=leptos::children::ToChildren::to_children(move || rc!("header", row))
                hero=leptos::children::ToChildren::to_children(move || rc!("hero"))
                footer=leptos::children::ToChildren::to_children(move || rc!("footer", row))
            >
                {rc!("main")}
            </MarketingLayout>
        }.into_any(),

        ActiveLayout::Fullscreen => view! {
            <FullscreenLayout
                header_zone_id=zid("header")
                main_zone_id=zid("main")
                header=leptos::children::ToChildren::to_children(move || rc!("header", row))
            >
                {rc!("main")}
            </FullscreenLayout>
        }.into_any(),

        ActiveLayout::SplitView => view! {
            <SplitViewLayout
                ratio=SplitRatio::Equal
                left_zone_id=zid("left")
                right_zone_id=zid("right")
                left=leptos::children::ToChildren::to_children(move || rc!("left"))
                right=leptos::children::ToChildren::to_children(move || rc!("right"))
            />
        }.into_any(),

        ActiveLayout::Wizard => view! {
            <WizardLayout
                header_zone_id=zid("header")
                stepper_zone_id=zid("stepper")
                main_zone_id=zid("main")
                footer_zone_id=zid("footer")
                header=leptos::children::ToChildren::to_children(move || rc!("header", row))
                stepper=leptos::children::ToChildren::to_children(move || rc!("stepper", row))
                footer=leptos::children::ToChildren::to_children(move || rc!("footer", row))
            >
                {rc!("main")}
            </WizardLayout>
        }.into_any(),

        ActiveLayout::Section => view! {
            <Section
                header_zone_id=zid("header")
                main_zone_id=zid("main")
                footer_zone_id=zid("footer")
                header=leptos::children::ToChildren::to_children(move || rc!("header", row))
                footer=leptos::children::ToChildren::to_children(move || rc!("footer", row))
            >
                {rc!("main")}
            </Section>
        }.into_any(),

        ActiveLayout::PageSingle => view! {
            <PageLayout variant=PageLayoutVariant::Single main_zone_id=zid("main")>
                {rc!("main")}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithSidebar => view! {
            <PageLayout
                variant=PageLayoutVariant::WithSidebar
                sidebar_zone_id=zid("sidebar")
                main_zone_id=zid("main")
                sidebar=leptos::children::ToChildren::to_children(move || rc!("sidebar"))
            >
                {rc!("main")}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageWithAside => view! {
            <PageLayout
                variant=PageLayoutVariant::WithAside
                aside_zone_id=zid("aside")
                main_zone_id=zid("main")
                aside=leptos::children::ToChildren::to_children(move || rc!("aside"))
            >
                {rc!("main")}
            </PageLayout>
        }.into_any(),

        ActiveLayout::PageSidebarAndAside => view! {
            <PageLayout
                variant=PageLayoutVariant::SidebarAndAside
                sidebar_zone_id=zid("sidebar")
                aside_zone_id=zid("aside")
                main_zone_id=zid("main")
                sidebar=leptos::children::ToChildren::to_children(move || rc!("sidebar"))
                aside=leptos::children::ToChildren::to_children(move || rc!("aside"))
            >
                {rc!("main")}
            </PageLayout>
        }.into_any(),
    }
}
