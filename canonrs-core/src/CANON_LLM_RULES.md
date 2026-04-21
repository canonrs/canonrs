# CanonRS — Complete LLM Rules & Conventions

> Static reference for LLM builder. Combined with llm_context.md for full context.
> Last updated: auto

---

## 1. Architecture — Layer Flow
Primitive → UI → Boundary → Block → Layout → Page

Each layer has ONE responsibility. No layer knows the layers above it.

| Layer      | Location                         | Slot type            | Rule                            |
|------------|----------------------------------|----------------------|---------------------------------|
| Primitive  | canonrs_core::primitives         | —                    | HTML + ARIA + data-rs-* only    |
| UI         | canonrs::ui::{id}::{Name}Ui      | Children             | 1:1 mapping over primitive      |
| Boundary   | canonrs::ui::{id}::{Name}        | Children             | typed defaults, delegates to UI |
| Block      | canonrs::blocks::{id}            | Option<ChildrenFn>   | semantic composition only       |
| Layout     | canonrs::layouts::{id}           | Option<ChildrenFn>   | page structure only             |
| Page       | product src/pages/               | ChildrenFn           | final composition               |

---

## 2. Cargo Setup

canonrs = { path = "../../packages-rust/rs-canonrs/canonrs", default-features = false }

features:
  canonrs/ssr      # server-side rendering
  canonrs/hydrate  # client-side hydration

---

## 3. Import Patterns

### Layout Primitives
Primitives are exported from canonrs:: root — NOT from primitives::layout::*

    use canonrs::{
        StackPrimitive, StackGap, StackAlign, StackDirection,
        FlexPrimitive, FlexDirection, FlexAlign, FlexJustify, FlexGap,
        GridPrimitive, GridCols, GridGap,
        SpacerPrimitive,
    };
    // Container — exception, has its own submodule:
    use canonrs::layout::container::{ContainerPrimitive, ContainerSize};
    // Center — exported from canonrs:: root like other primitives:
    use canonrs::CenterPrimitive;
    use canonrs::CenterMode;

### Layouts

    use canonrs::layouts::marketing::MarketingLayout;
    use canonrs::layouts::dashboard::DashboardLayout;
    use canonrs::layouts::page_layout::{PageLayout, PageLayoutVariant};
    use canonrs::layouts::split_view::{SplitViewLayout, SplitRatio};
    use canonrs::layouts::three_pane::ThreePaneLayout;
    use canonrs::layouts::fullscreen::FullscreenLayout;
    use canonrs::layouts::wizard::WizardLayout;

### Blocks

    use canonrs::blocks::hero::Hero;
    use canonrs::blocks::section::SectionBlock;
    use canonrs::blocks::card::{CardBlock, CardVariant};
    use canonrs::blocks::page_header::PageHeader;
    use canonrs::blocks::stat_group::StatGroupBlock;
    use canonrs::blocks::data_table::DataTableBlock;

### UI Components

    use canonrs::ui::button::{Button, ButtonVariant, ButtonSize};
    use canonrs::ui::badge::Badge;
    use canonrs::ui::input::Input;
    use canonrs::ui::hero::{HeroTitle, HeroSubtitle, HeroDescription};
    use canonrs::ui::stat::{Stat, StatHeader, StatValue, StatDelta, StatSize, StatTrend};
    use canonrs::ui::tabs::{TabsRoot, TabsTrigger, TabsContent};
    use canonrs::ui::select::{Select, SelectItem};
    use canonrs::ui::checkbox::Checkbox;
    use canonrs::ui::progress::Progress;
    use canonrs::ui::skeleton::Skeleton;
    use canonrs::ui::spinner::Spinner;
    use canonrs::ui::alert::{Alert, AlertVariant};
    use canonrs::ui::inline_notice::InlineNotice;
    use canonrs::ui::inline_meta::{InlineMeta, InlineMetaLabel, InlineMetaValue};
    use canonrs::ui::tooltip::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
    use canonrs::ui::switch::Switch;
    use canonrs::ui::copy_button::CopyButton;
    use canonrs::ui::toolbar::{Toolbar, ToolbarItem, ToolbarSeparator};
    use canonrs::ui::button_group::ButtonGroup;
    use canonrs::ui::loading_overlay::LoadingOverlay;
    use canonrs::ui::scroll_area::ScrollArea;
    use canonrs::ui::markdown::{MarkdownSurface, render_markdown};
    use canonrs::ui::nav_item::{NavItem, NavGroup};
    use canonrs::ui::form::Form;
    use canonrs::ui::field::Field;

### Top-level enum re-exports

    use canonrs::{
        BadgeVariant,        // Default, Primary, Success, Warning, Destructive, Outline
        ActivityState,       // Active, Inactive
        InlineNoticeVariant,
        ToggleState,         // On, Off
    };

### Slot macro (required for blocks)

    use canonrs::slot;

---

## 4. Slot Syntax — CRITICAL DISTINCTION

Layout slots and block slots are both Option<ChildrenFn> but use DIFFERENT syntax.

### Blocks — use slot!() macro

    body=slot!(|| view! { <MyContent /> }.into_any())
    body=slot!(move || view! { <MyContent value=signal.get() /> }.into_any())

### Layouts — use leptos::children::ToChildren::to_children()

    content=leptos::children::ToChildren::to_children(|| view! { <MyContent /> })
    content=leptos::children::ToChildren::to_children(move || view! { <MyContent value=val /> })

    WRONG — slot!() does NOT work for layout slots:
    content=slot!(|| view! { <MyContent /> }.into_any())  // compile error

### Why the difference?
slot!() wraps in Arc<dyn Fn() -> AnyView> and calls .into_any().
ToChildren::to_children() does NOT call .into_any().
Mixing them causes E0308 mismatched types.

---

## 5. Type Inference Rule — E0282

When a layout slot contains complex nested components, the Rust compiler cannot
infer the view type. Always extract complex layout slot content into a
sub-component:

    // WRONG — causes E0282 cannot infer type
    content=leptos::children::ToChildren::to_children(|| view! {
        <StackPrimitive gap=StackGap::Lg>
            <Alert ... />
            <SectionBlock ... />
        </StackPrimitive>
    })

    // CORRECT — extract to #[component]
    #[component]
    fn PageContent() -> impl IntoView {
        view! {
            <StackPrimitive gap=StackGap::Lg>
                <Alert ... />
                <SectionBlock ... />
            </StackPrimitive>
        }
    }

    content=leptos::children::ToChildren::to_children(|| view! { <PageContent/> })

Block slots (slot!()) do NOT have this problem — only layout slots.

---

## 6. Page Structure Pattern

    use leptos::prelude::*;
    use canonrs::slot;
    use canonrs::layouts::marketing::MarketingLayout;
    use canonrs::blocks::hero::Hero;
    use canonrs::blocks::section::SectionBlock;
    use canonrs::blocks::page_header::PageHeader;
    use canonrs::layout::container::{ContainerPrimitive, ContainerSize};
    use canonrs::{GridPrimitive, GridCols, GridGap, StackPrimitive, StackGap};
    use canonrs::ui::button::{Button, ButtonVariant, ButtonSize};
    use canonrs::ui::hero::{HeroTitle, HeroSubtitle, HeroDescription};

    #[component]
    fn PageFeatures() -> impl IntoView {
        view! {
            <ContainerPrimitive size=ContainerSize::Lg>
                <SectionBlock
                    body=slot!(|| view! {
                        <GridPrimitive cols=GridCols::Three gap=GridGap::Md>
                            // cards
                        </GridPrimitive>
                    }.into_any())
                />
            </ContainerPrimitive>
        }
    }

    #[component]
    pub fn MyPage() -> impl IntoView {
        view! {
            <MarketingLayout
                hero=leptos::children::ToChildren::to_children(|| view! {
                    <Hero
                        content=slot!(|| view! {
                            <HeroTitle>"Title"</HeroTitle>
                            <HeroSubtitle>"Subtitle"</HeroSubtitle>
                        }.into_any())
                        actions=slot!(|| view! {
                            <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>"CTA"</Button>
                        }.into_any())
                    />
                })
                content=leptos::children::ToChildren::to_children(|| view! {
                    <PageFeatures/>
                })
            />
        }
    }

---

## 7. Layout Primitive Decision Guide

| Primitive   | Use when                                             | Default            |
|-------------|------------------------------------------------------|--------------------|
| Stack       | Linear layout vertical or horizontal — DEFAULT       | direction=vertical |
| Flex        | Need justify, wrap, or full flex control             | direction=row      |
| Grid        | Multi-column content: cards, stats, thumbnails       | cols=auto          |
| Container   | Max-width wrapper at page or section level           | size=lg            |
| Center      | Single centered element: login, empty, modal         | mode=both          |
| Spacer      | Push siblings apart inside Flex or horizontal Stack  | —                  |

Rules:
- Default to Stack — reach for Flex only when Stack is not enough
- Grid is for content grids, NOT page layout
- Always wrap page content with Container for max-width control
- Spacer has no children — only use inside Flex or horizontal Stack
- Use Stack inside block body slots in 80% of cases

Correct nesting pattern:

    body=slot!(|| view! {
        <StackPrimitive gap=StackGap::Md>
            <Alert variant=AlertVariant::Warning title="..." description="..." />
            <Progress value=67.0 />
            <DataTableBlock ... />
        </StackPrimitive>
    }.into_any())

    WRONG — UI directly inside block body with no primitive:
    body=slot!(|| view! {
        <Alert ... />
        <Progress ... />
    }.into_any())

---

## 8. BadgeVariant — Available Values

    BadgeVariant::Default
    BadgeVariant::Primary
    BadgeVariant::Success
    BadgeVariant::Warning
    BadgeVariant::Destructive
    BadgeVariant::Outline

    DOES NOT EXIST: BadgeVariant::Secondary — use Outline instead

---

## 9. Canon Rules — Critical

### CR-338 — boundary_type required
Every component boundary MUST declare boundary_type in builder.yaml:
  passthrough — no logic, direct prop delegation
  init        — normalizes props, delegates to UI
  interaction — has WASM runtime behavior

### CR-339 — dynamic class forbidden
    WRONG:  class=move || if active { "active" } else { "" }
    CORRECT: data-rs-state="active"

### CR-340 — pseudo-class as state forbidden
    WRONG:  button:hover { }
    CORRECT: [data-rs-state~="hover"] { }

### CR-341 — hardcoded values forbidden in CSS
    WRONG:  color: #ff0000; padding: 16px;
    CORRECT: color: var(--theme-text-primary); padding: var(--space-4);

### CR-342 — display:none without state forbidden
    WRONG:  .panel { display: none; }
    CORRECT: [data-rs-panel][data-rs-state~="closed"] { display: none; }

### CR-345 — aria attributes as CSS selectors forbidden
    WRONG:  [aria-expanded] { }
    CORRECT: [data-rs-state~="open"] { }

### CR-348 — data-rs-state must use ~= not =
    WRONG:  [data-rs-state="open"] { }
    CORRECT: [data-rs-state~="open"] { }

### CR-350 — data-rs-uid required in all primitives
    data-rs-uid=crate::infra::uid::generate("prefix")

### CR-360 — UI must not use raw HTML
    WRONG:  view! { <div>"content"</div> }
    CORRECT: view! { <CardPrimitive>"content"</CardPrimitive> }

### CR-361 — UI must not create signals
    WRONG:  let (value, set_value) = signal(false);

### CR-370 — Boundary must not create signals
    WRONG:  let state = RwSignal::new(false);

### CR-371 — Boundary must not have reactive closures
    WRONG:  move || { ... }

### CR-372 — Boundary must not have business logic
    WRONG:  use_navigate(); fetch(...); spawn_local(...);

### CR-380 — Preview must use boundary not UI
    WRONG:  use super::button_ui::Button;
    CORRECT: use super::button_boundary::Button;

### CR-382 — Preview must not create signals
### CR-383 — Preview must not fetch data

---

## 10. State Engine Contract

State is ALWAYS expressed via data-rs-state — never via class, never inline style.

    use canonrs_core::infra::state_engine::{disabled_attrs, loading_attrs, visibility_attrs};

    let d = disabled_attrs(disabled);
    let s = visibility_attrs(state);

    data-rs-state=s.data_rs_state
    aria-disabled=d.aria_disabled
    disabled=d.disabled

    WRONG: get_attribute("data-rs-disabled").as_deref() == Some("disabled")
    CORRECT: has_attribute("data-rs-disabled")

---

## 11. Anti-patterns Summary

    WRONG:  <div style="display:flex;gap:1rem;">
    CORRECT: <FlexPrimitive gap=FlexGap::Md>

    WRONG:  <button class="btn btn-primary btn-md">
    CORRECT: <Button variant=ButtonVariant::Primary size=ButtonSize::Md>

    WRONG:  header=slot!(|| view! { <span>"Title"</span> }.into_any())
    CORRECT: header=slot!(|| view! { <PageHeader title=slot!(|| view! { "Title" }.into_any()) /> }.into_any())

    WRONG:  <Card variant="outlined">
    CORRECT: <Card variant=CardVariant::Outlined>

    WRONG:  <ButtonPrimitive>  (raw primitive in page)
    CORRECT: <Button>          (always use boundary in pages)

    WRONG:  Two layouts with <main> on same page
    CORRECT: Only one layout with <main> per page
