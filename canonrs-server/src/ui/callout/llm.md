# Callout — LLM Context

## Identity

### component_id
callout

### type
feedback

### family
feedback

### pillar
contextual emphasis

### canonical_import
canonrs::ui::callout

## Contract

### What the compiler enforces
Variant defines semantic role and behavior.

### What cannot drift
- Semantic meaning
- Accessibility roles

### What is guaranteed at compile time
- Correct variant usage

## Definition

### Precise technical definition
Callout is a composable feedback container that highlights contextual information with enforced semantics.

### Distinction from HTML element
Unlike div, it encodes meaning and accessibility.

### Distinction from similar components
Alert is system-driven; Callout is contextual.

## API

### Props
- variant: CalloutVariant
- class: String

### Variants
- Default
- Info
- Warning
- Destructive

### Sizes
None

### States
Static

## Patterns

### When to use
Highlight contextual information.

### When NOT to use
System alerts or transient messages.

### Anti-patterns
Using plain div for callouts.

## Composition

### Works with
Title, description, icon.

### Does not work with
Unstructured content.

### Requires
Optional parts only.

## Governance

### Canon Rules
CR-001, CR-004

### Violations prevented
- Incorrect semantics
- Accessibility issues

### SSR safety
Deterministic rendering.

### Hydration safety
No runtime logic.

## Examples

### Minimal
```rust
view! {
  <Callout>"Info"</Callout>
}
```

### With all props
```rust
view! {
  <Callout variant=CalloutVariant::Warning>
    <CalloutTitle>"Warning"</CalloutTitle>
    <CalloutDescription>"Be careful"</CalloutDescription>
  </Callout>
}
```

### Common mistake + correction
Wrong: <div class="callout">  
Correct: <Callout>
