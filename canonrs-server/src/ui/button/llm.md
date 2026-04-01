# Button — LLM Context

## Identity

### component_id
button

### type
interactive

### family
action

### pillar
interaction

### canonical_import
canonrs::ui::button::Button

## Contract

### What the compiler enforces
- variant enum
- size enum
- disabled state

### What cannot drift
- styling semantics
- interaction behavior

### What is guaranteed at compile time
- accessibility
- SSR safety

## Definition

### Precise technical definition
Button is a typed interactive component enforcing action semantics. It encodes behavior via primitives. It prevents invalid combinations. It ensures deterministic rendering.

### Distinction from HTML element
HTML button is flexible but unsafe. Button enforces constraints. It guarantees consistency. It removes ambiguity.

### Distinction from similar components
Link is navigation. Button is action. IconButton is compact. Button is general-purpose.

## API

### Props
- variant: ButtonVariant = Primary
- size: ButtonSize = Md
- disabled: DisabledState = Enabled
- class: String = ""
- aria_label: Option<String>

### Variants
- Default
- Primary
- Secondary
- Outline
- Ghost
- Link
- Destructive

### Sizes
- Xs
- Sm
- Md
- Lg
- Xl

### States
- enabled
- disabled

## Patterns

### When to use
- form submit
- actions
- CTA

### When NOT to use
- navigation links
- passive elements

### Anti-patterns
- using button for navigation
- mixing variants

## Composition

### Works with
- ButtonGroup
- Card
- Form

### Does not work with
- inline text elements

### Requires
- ButtonPrimitive

## Governance

### Canon Rules
- CR-001
- CR-004
- CR-148

### Violations prevented
- style drift
- invalid states
- accessibility issues

### SSR safety
attributes define behavior

### Hydration safety
no runtime mutation

## Examples

### Minimal
```rust
view! { <Button>"Submit"</Button> }
```

### With all props
```rust
view! {
  <Button variant=ButtonVariant::Primary size=ButtonSize::Md disabled=DisabledState::Enabled>
    "Submit"
  </Button>
}
```

### Common mistake + correction
```rust
// ❌ wrong
<button class="btn">

// ✅ correct
<Button>"Submit"</Button>
```
