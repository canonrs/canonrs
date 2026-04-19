# HOW TO USE BLOCKS AND PRIMITIVES

## The Flow

    Primitive → UI → Boundary → Block → Layout → Page

Each layer has one responsibility. No layer knows the layers above it.

---

## Layers

### Primitive (`canonrs_core::primitives`)
Pure HTML + ARIA + `data-rs-*` attributes. Zero logic. Zero style.
Use only when building a new UI component from scratch.

    use canonrs_core::primitives::CardPrimitive;

### UI (`canonrs::ui`)
1:1 mapping over the primitive. Typed props, no logic, no state.
Use when composing inside a Boundary.

    use canonrs::ui::card::{Card, CardHeader, CardContent, CardFooter};

### Boundary (`canonrs::ui`)
Public API of the component. Applies typed defaults. Delegates to UI.
This is what pages and blocks should import for atomic components.

    <Card>"content"</Card>
    <CardHeader><CardTitle>"Title"</CardTitle></CardHeader>

### Block (`canonrs::blocks`)
Semantic composition of Boundaries. Slots are always `Option<ChildrenFn>`.
Use when a pattern composes multiple UI regions into a reusable context.

    use canonrs::blocks::card::{CardBlock, CardVariant};

    <CardBlock
        variant=CardVariant::Outlined
        header=slot!(|| view! { <CardHeader><CardTitle>"Title"</CardTitle></CardHeader> }.into_any())
        content=slot!(|| view! { <CardContent>"Body"</CardContent> }.into_any())
        footer=slot!(|| view! { <CardFooter>"Action"</CardFooter> }.into_any())
    />

### Layout Primitive (`canonrs::primitives::layout`)
Structural layout containers. Use inside blocks and pages for arrangement.

    use canonrs::primitives::layout::grid::{GridPrimitive as Grid, GridCols};
    use canonrs::primitives::layout::stack::{StackPrimitive, StackDirection};

    <Grid cols=GridCols::Three>
        <CardBlock ... />
        <CardBlock ... />
        <CardBlock ... />
    </Grid>

---

## Rules

| Layer     | Slot type          | Rule                              |
|-----------|--------------------|-----------------------------------|
| Primitive | —                  | HTML + ARIA only                  |
| UI        | `Children`         | 1:1 mapping, no logic             |
| Boundary  | `Children`         | typed defaults, delegates to UI   |
| Block     | `Option<ChildrenFn>` | semantic composition only       |
| Layout    | `Option<ChildrenFn>` | page structure only             |
| Page      | `ChildrenFn`       | final composition                 |

## Anti-patterns

    // ❌ inline style
    <div style="display:flex">

    // ❌ variant in Primitive
    CardPrimitive { variant: CardVariant }

    // ❌ slots in Boundary
    Card { header: Option<ChildrenFn> }

    // ❌ HTML in Block slots (use UI components)
    header=slot!(|| view! { <span>"Title"</span> }.into_any())

    // ✅ correct
    header=slot!(|| view! { <CardHeader><CardTitle>"Title"</CardTitle></CardHeader> }.into_any())








# Preview Review Process

## Objetivo
Revisar todos os arquivos `preview.rs` da camada UI para garantir conformidade com a arquitetura CanonRS.

## Diretório base
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-server/src/ui/

## Regras obrigatórias

### 1. Sempre usar Boundary (nunca UI direto)
✅ use super::button_boundary::Button;
❌ use super::button_ui::Button;

### 2. Nunca usar style inline
✅ <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
❌ <div style="display:flex;gap:1rem;">

### 3. Layout via primitives
✅ use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
✅ use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};
❌ <div data-rs-showcase-preview-row="">
❌ <div data-rs-showcase-preview-hero="">

### 4. Enums e tipos devem vir do boundary (re-exportados)
✅ use super::avatar_boundary::{Avatar, AvatarSize, AvatarShape};
❌ use super::avatar_ui::{AvatarSize, AvatarShape};

### 5. Blocks obrigatórios
- `Hero` → sempre via `crate::blocks::hero::hero_block::Hero`
- `Card` com regiões → sempre via `crate::blocks::card::{CardBlock, CardVariant}`

### 6. Boundary deve ser 1:1 com o UI
- Mesmos props
- Sem lógica adicional
- Re-exporta todos os tipos necessários

### 7. Import de outros componentes dentro do preview
✅ use crate::ui::button::button_boundary::Button;
❌ use crate::ui::button::Button; (sem especificar boundary)
❌ use crate::ui::button::button_ui::Button;






gesture

resizable
slider
carousel
scroll_area



selection

select
combobox
radio_group
toggle_group
tree
color_picker



overlay

alert_dialog
confirm_dialog
context_menu
dialog
drawer
dropdown_menu
hover_card
modal
popover
sheet



data

chart
virtual_list
list_item
data_table



content
  
markdown
copy_button



nav

accordion
tabs
pagination
toolbar
link_group
breadcrumb
menubar
sidebar





/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/center.rs
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/container.rs
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/flex.rs
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/grid.rs
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/mod.rs
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/spacer.rs
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-core/src/primitives/layout/stack.rs