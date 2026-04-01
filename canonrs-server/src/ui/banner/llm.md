# Banner — LLM Context

## Identity

### component_id
banner

### type
feedback

### family
feedback

### pillar
persistent messaging

### canonical_import
canonrs::ui::banner

## Contract

### What the compiler enforces
Banner enforces structure and accessibility via variant and state.

### What cannot drift
- Visibility behavior
- ARIA semantics

### What is guaranteed at compile time
- Correct structure
- Accessible rendering

## Definition

### Precise technical definition
Banner is a composable feedback container for persistent, page-level messaging with enforced semantics.

### Distinction from HTML element
Unlike a div, Banner encodes accessibility and visibility behavior.

### Distinction from similar components
Toast is transient; Banner is persistent.

## API

### Props
- variant: BannerVariant
- class: String

### Variants
- Info
- Warning
- Destructive

### Sizes
None

### States
- Visible
- Hidden

## Patterns

### When to use
Persistent page-level messages.

### When NOT to use
Transient notifications.

### Anti-patterns
Using div without accessibility semantics.

## Composition

### Works with
Content, actions, close controls.

### Does not work with
Unstructured children.

### Requires
BannerContent for meaningful usage.

## Governance

### Canon Rules
CR-001, CR-004

### Violations prevented
- Missing accessibility roles
- Inconsistent visibility

### SSR safety
Deterministic rendering.

### Hydration safety
State encoded in attributes.

## Examples

### Minimal
```rust
view! {
  <Banner>
    <BannerContent>"Message"</BannerContent>
  </Banner>
}
```

### With all props
```rust
view! {
  <Banner variant=BannerVariant::Warning>
    <BannerContent>"Warning"</BannerContent>
  </Banner>
}
```

### Common mistake + correction
Wrong: <div class="banner">  
Correct: <Banner>
