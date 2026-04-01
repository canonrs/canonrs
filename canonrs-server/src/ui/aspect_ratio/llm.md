# Aspect Ratio — LLM Context

## Identity

### component_id
aspect-ratio

### type
layout

### family
display

### pillar
layout stability

### canonical_import
canonrs::ui::aspect_ratio

## Contract

### What the compiler enforces
Ratio must be defined through numeric props.

### What cannot drift
Layout proportions.

### What is guaranteed at compile time
Consistent rendering regardless of content.

## Definition

### Precise technical definition
AspectRatio is a container that enforces a fixed width-to-height ratio using structural constraints.

### Distinction from HTML element
Unlike CSS hacks, it uses primitives to enforce layout.

### Distinction from similar components
Resizable allows dynamic resizing; AspectRatio enforces fixed ratio.

## API

### Props
- ratio_w: f32
- ratio_h: f32
- class: String

### Variants
None

### Sizes
None

### States
Static

## Patterns

### When to use
Media containers.

### When NOT to use
Flexible layouts.

### Anti-patterns
Padding-based ratio hacks.

## Composition

### Works with
Images, videos.

### Does not work with
Dynamic resizing logic.

### Requires
Children content.

## Governance

### Canon Rules
CR-001, CR-004

### Violations prevented
Layout shift.

### SSR safety
Deterministic layout.

### Hydration safety
No runtime calculations.

## Examples

### Minimal
```rust
view! {
  <AspectRatio>
    <img src="img.png" />
  </AspectRatio>
}
```

### With all props
```rust
view! {
  <AspectRatio ratio_w=16.0 ratio_h=9.0>
    <img src="img.png" />
  </AspectRatio>
}
```

### Common mistake + correction
Wrong: padding-top hack  
Correct: AspectRatio component
