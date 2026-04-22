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


