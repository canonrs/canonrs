# Animate — LLM Context

## Identity

### component_id
animate

### type
utility

### family
display

### pillar
motion

### canonical_import
canonrs::ui::animate::Animate

## Contract

### What the compiler enforces
- Valid animation enum
- Valid easing enum
- No arbitrary classes

### What cannot drift
- animation type
- easing
- duration structure

### What is guaranteed at compile time
- SSR-safe animation attributes
- deterministic behavior

## Definition

### Precise technical definition
Animate is a wrapper that applies typed animation behavior. It encodes motion into DOM attributes. It avoids class-based styling. It ensures consistent animation execution.

### Distinction from HTML element
HTML uses class-based animations. Animate enforces typed constraints. It guarantees consistency. It removes ambiguity.

### Distinction from similar components
Carousel manages grouped motion. Animate handles single element transitions. It is simpler and more controlled. It does not manage sequences.

## API

### Props
- animation: AnimationName = FadeIn
- easing: AnimationEasing = EaseInOut
- duration: String = ""
- delay: String = ""
- class: String = ""

### Variants
- FadeIn
- SlideIn

### Sizes
- duration-based

### States
- active
- idle

## Patterns

### When to use
- page transitions
- modal animations
- element entry

### When NOT to use
- complex sequences
- state-driven animations

### Anti-patterns
- manual CSS animations → use Animate
- inline styles → use typed props

## Composition

### Works with
- Modal
- Dialog
- Card

### Does not work with
- navigation routing systems

### Requires
- AnimatePrimitive

## Governance

### Canon Rules
- CR-001
- CR-004

### Violations prevented
- inconsistent animation
- class drift
- hydration mismatch

### SSR safety
Attributes define animation state.

### Hydration safety
No runtime mutation required.

## Examples

### Minimal
```rust
view! { <Animate>"Content"</Animate> }
```

### With all props
```rust
view! {
  <Animate animation=AnimationName::FadeIn easing=AnimationEasing::EaseInOut duration="300ms" delay="100ms">
    "Content"
  </Animate>
}
```

### Common mistake + correction
```rust
// ❌ wrong
<div class="fade-in">

// ✅ correct
<Animate>"Content"</Animate>
```
